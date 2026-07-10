use std::{io, time::Duration};
use tokio::sync::mpsc;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph},
};
use common::{ProgramSummary, ProcessStatus};
use crate::handlers::Context;

// + App State +

struct App {
    items: Vec<ProgramSummary>,
    table_state: TableState,
    is_loading: bool,
    last_error: Option<String>,
}

impl App {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            table_state: TableState::default(),
            is_loading: true,
            last_error: None,
        }
    }

    fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}

// + Entry Point +

pub async fn run(ctx: &Context) -> anyhow::Result<()> {
    // 1. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Setup Data Channel
    let (tx, mut rx) = mpsc::channel::<Result<Vec<ProgramSummary>, String>>(10);

    // Background Fetcher
    let client = ctx.client.clone();
    let url = format!("{}/api/programs", ctx.base_url);

    tokio::spawn(async move {
        loop {
            // Fetch data
            let res = match client.get(&url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.json::<Vec<ProgramSummary>>().await {
                            Ok(data) => Ok(data),
                            Err(e) => Err(format!("Parse Error: {}", e)),
                        }
                    } else {
                        Err(format!("API Error: {}", resp.status()))
                    }
                },
                Err(e) => Err(format!("Network Error: {}", e)),
            };

            // Send to UI
            if tx.send(res).await.is_err() {
                break; // UI closed
            }

            // Refresh rate
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    // 3. Main UI Loop
    let mut app = App::new();
    let tick_rate = Duration::from_millis(100); // UI responsive rate

    loop {
        // Draw
        terminal.draw(|f| ui(f, &mut app))?;

        // Handle Input (Non-blocking check)
        if event::poll(tick_rate)?
            && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => break,
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        _ => {}
                    }
                }

        // Handle Data Update
        while let Ok(msg) = rx.try_recv() {
            app.is_loading = false;
            match msg {
                Ok(mut items) => {
                    app.last_error = None;
                    // Sort to keep display stable
                    items.sort_by(|a, b| a.name.cmp(&b.name));
                    app.items = items;
                    // Ensure selection is valid
                    if app.table_state.selected().is_none() && !app.items.is_empty() {
                        app.table_state.select(Some(0));
                    }
                },
                Err(e) => app.last_error = Some(e),
            }
        }
    }

    // 4. Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

// + UI Rendering +

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Table
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    render_header(f, app, chunks[0]);
    render_table(f, app, chunks[1]);
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let (total, running, error) = app.items.iter().fold((0, 0, 0), |(t, r, e), p| {
        let is_run = matches!(p.status, ProcessStatus::Running | ProcessStatus::Healthy);
        let is_err = matches!(p.status, ProcessStatus::Fatal | ProcessStatus::Backoff);
        (t + 1, r + if is_run { 1 } else { 0 }, e + if is_err { 1 } else { 0 })
    });

    let status_text = if let Some(err) = &app.last_error {
        Line::from(vec![Span::styled(format!(" Error: {} ", err), Style::default().bg(Color::Red).fg(Color::White))])
    } else {
        Line::from(vec![
            Span::raw(" Total: "), Span::styled(total.to_string(), Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled(" Running ", Style::default().fg(Color::Green)),
            Span::styled(running.to_string(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::styled(" Fatal/Backoff ", Style::default().fg(Color::Red)),
            Span::styled(error.to_string(), Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ])
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(" Super Process Manager (TUI) ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)));

    let p = Paragraph::new(status_text)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(p, area);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);

    let header = Row::new(vec![
        Cell::from("ID"),
        Cell::from("Name"),
        Cell::from("Status"),
        Cell::from("PID"),
        Cell::from("CPU %"),
        Cell::from("Mem"),
        Cell::from("Uptime"),
    ])
    .style(header_style)
    .height(1);

    let rows: Vec<Row> = app.items.iter().map(|p| {
        let status_style = match p.status {
            ProcessStatus::Healthy => Style::default().fg(Color::Green),
            ProcessStatus::Running => Style::default().fg(Color::Green),
            ProcessStatus::Fatal => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ProcessStatus::Backoff => Style::default().fg(Color::Yellow),
            ProcessStatus::Stopped => Style::default().fg(Color::DarkGray),
            _ => Style::default(),
        };

        let pid = p.pid.map(|x| x.to_string()).unwrap_or_else(|| "-".to_string());
        let cpu = p.cpu_usage.map(|x| format!("{:.1}", x)).unwrap_or_else(|| "-".to_string());
        let mem = p.mem_usage.map(|x| {
             if x > 1024 * 1024 { format!("{:.1} MB", x as f64 / 1024.0 / 1024.0) }
             else { format!("{:.0} KB", x as f64 / 1024.0) }
        }).unwrap_or_else(|| "-".to_string());

        let uptime = p.uptime_sec.map(|s| {
            let h = s / 3600;
            let m = (s % 3600) / 60;
            let sec = s % 60;
            format!("{:02}:{:02}:{:02}", h, m, sec)
        }).unwrap_or_else(|| "-".to_string());

        // ID Short
        let id_short = p.id.to_string().chars().take(8).collect::<String>();

        Row::new(vec![
            Cell::from(id_short),
            Cell::from(p.name.clone()),
            Cell::from(format!("{:?}", p.status)).style(status_style),
            Cell::from(pid),
            Cell::from(cpu),
            Cell::from(mem),
            Cell::from(uptime),
        ])
    }).collect();

    let t = Table::new(
        rows,
        [
            Constraint::Length(10), // ID
            Constraint::Percentage(20), // Name
            Constraint::Percentage(15), // Status
            Constraint::Length(8), // PID
            Constraint::Length(8), // CPU
            Constraint::Length(12), // Mem
            Constraint::Percentage(15), // Uptime
        ]
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" Processes "))
    .row_highlight_style(selected_style)
    .highlight_symbol(">> ");

    f.render_stateful_widget(t, area, &mut app.table_state);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let info = Paragraph::new("Quit: q/Esc | Select: j/k/↑/↓ | Refresh: 1s")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(info, area);
}
