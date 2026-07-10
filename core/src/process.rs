use common::ProgramConfig;
use std::process::Stdio;
use tokio::process::{Child, Command};
use std::collections::HashMap;
#[cfg(unix)]
use nix::unistd::User;


/// Build a Tokio Command from config and spawn the process.
pub fn spawn_process(config: &ProgramConfig, extra_envs: &HashMap<String, String>) -> anyhow::Result<Child> {
    let mut cmd = Command::new(&config.command);

    // 1. Arguments
    cmd.args(&config.args);

    // 2. Environment variables
    cmd.envs(&config.env);

    // Inject system env (SUPER_ID, SUPER_NAME, etc.)
    cmd.envs(extra_envs);

    // 3. Working directory
    if let Some(dir) = &config.cwd {
        cmd.current_dir(dir);
    }

    // 4. User switching
    if let Some(username) = &config.user {
        #[cfg(unix)]
        {
            if let Some(user) = User::from_name(username)? {
                use std::ffi::CString;
                let uid = user.uid.as_raw();
                let gid = user.gid.as_raw();
                let username_c = CString::new(username.clone())
                    .map_err(|_| anyhow::anyhow!("Invalid username string"))?;

                unsafe {
                    cmd.pre_exec(move || {
                        let c_user = username_c.as_ptr();
                        if libc::initgroups(c_user, gid as _) != 0 {
                            return Err(std::io::Error::last_os_error());
                        }
                        Ok(())
                    });
                }
                cmd.gid(gid);
                cmd.uid(uid);
            } else {
                return Err(anyhow::anyhow!("User '{}' not found on this system", username));
            }
        }

        #[cfg(not(unix))]
        {
            tracing::warn!("User switching (su) is not supported on non-Unix systems. Ignoring user='{}'.", username);
        }
    }

    // New process group: child and descendants share one PGID (equals child PID).
    // Windows does not support process_group.
    #[cfg(unix)]
    cmd.process_group(0);

    // 5. Pipes
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // 6. Daemon mode (optional)
    // cmd.kill_on_drop(false);

    let child = cmd.spawn()?;
    Ok(child)
}
