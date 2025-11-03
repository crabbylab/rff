use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use rff::{
    config::Config, error::AppError, fs::walker::FileWalker, input::InputReader, search::Matcher,
};
use std::io::stdout;
use std::process::Command;

fn main() -> Result<(), AppError> {
    let config = Config::try_parse()?;
    let walker = FileWalker::new(&config);
    let mut paths: Vec<String> = walker.into_paths().collect::<Result<Vec<_>, _>>()?;

    if !config.all {
        paths.retain(|p| !p.starts_with("target/"));
    }

    let matcher = Matcher::new();
    let input = InputReader::new();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut query = String::new();
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        let ranked = matcher.rank(&query, &paths);
        let items: Vec<ListItem> = ranked
            .iter()
            .take(50)
            .map(|p| ListItem::new(p.as_str()))
            .collect();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)])
                .split(f.size());

            let input = Paragraph::new(Span::styled(
                format!("Query: {}", query),
                Style::default().fg(Color::Yellow),
            ))
            .block(Block::default().borders(Borders::ALL).title(" Input "));

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Results "))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Cyan),
                )
                .highlight_symbol("> ");

            f.render_widget(input, chunks[0]);
            f.render_stateful_widget(list, chunks[1], &mut list_state);
        })?;

        if let Some(event) = input.read()? {
            match event {
                rff::input::InputEvent::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    let i = list_state.selected().unwrap_or(0);
                    let len = ranked.len();
                    if len > 0 {
                        list_state.select(Some((i + 1).min(len - 1)));
                    }
                }
                rff::input::InputEvent::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    let i = list_state.selected().unwrap_or(0);
                    list_state.select(Some(i.saturating_sub(1)));
                }
                rff::input::InputEvent::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    query.push(c);
                }
                rff::input::InputEvent::Backspace => {
                    query.pop();
                }
                rff::input::InputEvent::Enter => {
                    if let Some(selected) = ranked.get(list_state.selected().unwrap_or(0)) {
                        disable_raw_mode()?;
                        terminal.backend_mut().execute(LeaveAlternateScreen)?;

                        let full_path = config.root.join(selected);
                        let status = Command::new(&config.editor)
                            .arg(&full_path)
                            .status()
                            .map_err(|_| AppError::EditorSpawn)?;

                        if !status.success() {
                            eprintln!("Editor failed: {:?}", status);
                        }
                    }
                    break;
                }

                rff::input::InputEvent::Esc => break,
                rff::input::InputEvent::Tick => {}
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    Ok(())
}
