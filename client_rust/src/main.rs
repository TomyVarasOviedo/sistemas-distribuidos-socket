mod ui;
use std::io;
use std::time::Duration;
use std::time::Instant;

use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use tungstenite::connect;
use tungstenite::handshake::client::generate_key;
use tungstenite::handshake::client::Request;
use ui::render;
use ui::App;

fn main() -> io::Result<()> {
    let request = Request::builder()
        .uri("ws://localhost:8000/ws")
        .header("Host", "localhost:8000")
        .header("Origin", "http://localhost:8000")
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-Websocket-Version", "13")
        .header("Sec-Websocket-Key", generate_key())
        .body(())
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Error al construir request: {}", e),
            )
        })?;

    let (socket, response) = connect(request)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Conexión fallida: {}", e)))?;

    if response.status() != 101 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Servidor rechazó la conexión: {}", response.status()),
        ));
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(socket);
    let tick_rate = Duration::from_millis(80);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| render(f, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    break;
                }
                match key.code {
                    KeyCode::Enter => app.submit(),
                    KeyCode::Char('?') => app.show_help = !app.show_help,
                    KeyCode::Esc => {
                        if app.show_help {
                            app.show_help = false;
                        } else {
                            app.clear_input();
                        }
                    }
                    KeyCode::Backspace => app.delete_char_before(),
                    KeyCode::Delete => app.delete_char_after(),
                    KeyCode::Left => app.move_left(),
                    KeyCode::Right => app.move_right(),
                    KeyCode::Home => app.cursor_pos = 0,
                    KeyCode::End => app.cursor_pos = app.input.len(),
                    KeyCode::Char(c) => app.insert_char(c),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.tick = app.tick.wrapping_add(1);
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
