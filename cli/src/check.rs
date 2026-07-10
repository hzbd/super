use colored::Colorize;
use common::config::ServerConfig;
use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};

/// Run configuration check command
pub fn run(file_path: Option<PathBuf>) -> anyhow::Result<()> {
    // 1. Locate config file
    let path = resolve_config_path(file_path)?;
    println!(
        "Checking configuration at: {}",
        path.display().to_string().cyan()
    );

    // 2. Read and parse TOML
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} Failed to read file: {}", "✖".red(), e);
            return Err(e.into());
        }
    };

    let config: ServerConfig = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} TOML Syntax Error: {}", "✖".red(), e);
            return Err(anyhow::anyhow!("Invalid config format"));
        }
    };

    println!("   Syntax:      {}", "OK".green());

    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // 3. Check server config (port availability and privileges)
    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    print!("   Server Addr: {} ... ", bind_addr);

    // Try binding the port to detect conflicts
    match TcpListener::bind(&bind_addr) {
        Ok(_) => {
            print!("{}", "Available".green());
        }
        Err(e) => {
            print!("{}", "Occupied".red());
            errors.push(format!(
                "Port {} is likely in use: {}",
                config.server.port, e
            ));
        }
    }

    // Privileged ports (<1024) require root
    if config.server.port < 1024 && config.server.port != 0 {
        #[cfg(unix)]
        if unsafe { libc::geteuid() } != 0 {
            print!(" {}", "(Non-Root Warning)".yellow());
            warnings.push(format!(
                "Port {} usually requires root privileges",
                config.server.port
            ));
        }
    }
    println!();

    // 4. Check log directory (write permission)
    let log_dir = &config.storage.log_dir;
    print!("   Log Dir:     {:?} ... ", log_dir);

    if log_dir.exists() {
        if log_dir.is_dir() {
            if is_writable(log_dir) {
                println!("{}", "Writable".green());
            } else {
                println!("{}", "Permission Denied".red());
                errors.push(format!("Log dir {:?} exists but is NOT writable", log_dir));
            }
        } else {
            println!("{}", "Error".red());
            errors.push(format!(
                "Log path {:?} exists but is not a directory",
                log_dir
            ));
        }
    } else {
        // Directory missing; check whether parent allows creation
        let (writable, ancestor) = check_ancestor_writable(log_dir);
        if writable {
            println!("{}", "OK (Writable)".green());
        } else {
            println!("{}", "Permission Denied".red());
            errors.push(format!(
                "Cannot create log dir under read-only ancestor: {:?}",
                ancestor
            ));
        }
    }

    // 5. Check data file (snapshot storage)
    let data_file = &config.storage.data_file;
    print!("   Data File:   {:?} ... ", data_file);

    if data_file.exists() {
        if data_file.is_file() {
            // File exists; check writability
            if is_writable(data_file) {
                println!("{}", "Writable".green());
            } else {
                // Check read-only attribute
                if let Ok(m) = fs::metadata(data_file) {
                    if m.permissions().readonly() {
                        println!("{}", "Read-only".red());
                        errors.push(format!("Data file {:?} is read-only", data_file));
                    } else {
                        println!("{}", "Permission Denied".red());
                        errors.push(format!("Data file {:?} is not writable", data_file));
                    }
                }
            }
        } else {
            println!("{}", "Error".red());
            errors.push(format!(
                "Data path {:?} exists but is not a file",
                data_file
            ));
        }
    } else {
        // File missing; check whether parent allows creation
        if let Some(parent) = data_file.parent() {
            let (writable, ancestor) = check_ancestor_writable(parent);
            if writable {
                println!("{}", "OK (Writable)".green());
            } else {
                println!("{}", "Permission Denied".red());
                errors.push(format!(
                    "Cannot create data file under read-only ancestor: {:?}",
                    ancestor
                ));
            }
        } else {
            println!("{}", "Error".red());
            errors.push(format!("Invalid data file path: {:?}", data_file));
        }
    }

    // 6. Check include config (glob patterns)
    if !config.include.files.is_empty() {
        println!(
            "   Includes:    Found {} patterns",
            config.include.files.len()
        );
        for pattern in &config.include.files {
            if glob::glob(pattern).is_err() {
                errors.push(format!("Invalid glob pattern: {}", pattern));
            } else {
                let count = glob::glob(pattern).unwrap().count();
                if count == 0 {
                    println!("     - '{}': No matching files (Warning)", pattern);
                } else {
                    println!("     - '{}': Matches {} files", pattern, count);
                }
            }
        }
    }

    // 7. Print summary
    if !warnings.is_empty() {
        println!("\n{}", "Warnings:".yellow().bold());
        for w in warnings {
            println!("   {} {}", "⚠️".yellow(), w);
        }
    }

    if errors.is_empty() {
        println!("\n{}", "✨ Configuration is VALID".green().bold());
        Ok(())
    } else {
        println!("\n{}", "Found Errors:".red().bold());
        for e in errors {
            println!("   {} {}", "✖".red(), e);
        }
        Err(anyhow::anyhow!("Configuration check failed"))
    }
}

/// Check whether a path is writable.
/// For files, try append open; for directories, create a temp file.
fn is_writable(path: &Path) -> bool {
    if path.is_dir() {
        let test_file = path.join(".perm_check_tmp");
        if fs::write(&test_file, "").is_ok() {
            fs::remove_file(&test_file).ok();
            return true;
        }
    } else {
        if fs::OpenOptions::new().append(true).open(path).is_ok() {
            return true;
        }
    }
    false
}

/// Walk up to the nearest existing ancestor and check write permission.
/// Returns (writable, existing_ancestor_path).
fn check_ancestor_writable(target_path: &Path) -> (bool, PathBuf) {
    let mut current = target_path.to_path_buf();

    // Path does not exist; walk up
    while !current.exists() {
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            // At root or invalid path; fall back to current directory
            return (is_writable(Path::new(".")), Path::new(".").to_path_buf());
        }
    }

    // Found an existing directory; check writability
    (is_writable(&current), current)
}

/// Resolve default config file path
fn resolve_config_path(user_path: Option<PathBuf>) -> anyhow::Result<PathBuf> {
    if let Some(p) = user_path {
        return Ok(p);
    }

    let candidates = vec!["super.toml", "conf/super.toml", "/etc/super/super.toml"];

    for c in candidates {
        let p = PathBuf::from(c);
        if p.exists() {
            return Ok(p);
        }
    }

    Err(anyhow::anyhow!(
        "Config file not found. Please specify with --file"
    ))
}
