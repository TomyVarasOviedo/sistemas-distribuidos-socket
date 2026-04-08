use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};
use tungstenite::{Message, WebSocket};

//UTILS
fn evaluate(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>, expr: &str) -> Result<f64, String> {
    socket
        .send(Message::Text(expr.to_string()))
        .map_err(|e| format!("Error al enviar {}", e))?;
    let msg = socket
        .read()
        .map_err(|e| format!("Error al recibir {}", e))?;

    match msg {
        Message::Text(text) => {
            let text = text.trim().to_string();
            if let Ok(v) = text.parse::<f64>() {
                Ok(v)
            } else {
                Err(text)
            }
        }
        Message::Binary(b) => {
            let text = String::from_utf8_lossy(&b).trim().to_string();
            text.parse::<f64>()
                .map_err(|_| format!("Respuesta binaria {}", text))
        }
        Message::Close(_) => Err("El servidor cerró la conexión".into()),
        _ => Err("Error inesperado".into()),
    }
}
fn fmt_result(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        let s = format!("{:.8}", v);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
//UTILS

struct HistoryEntry {
    expr: String,
    result: Result<f64, String>,
}
pub struct App {
    pub input: String,
    pub cursor_pos: usize,
    history: Vec<HistoryEntry>,
    pub tick: u64,
    pub show_help: bool,
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl App {
    pub fn new(socket: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        Self {
            input: String::new(),
            cursor_pos: 0,
            history: Vec::new(),
            tick: 0,
            show_help: false,
            socket,
        }
    }

    pub fn submit(&mut self) {
        let expr = self.input.trim().to_string();
        if expr.is_empty() {
            return;
        }
        let result = evaluate(&mut self.socket, &expr);
        self.history.push(HistoryEntry { expr, result });
        self.input.clear();
        self.cursor_pos = 0;
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor_pos, c);
        self.cursor_pos += c.len_utf8();
    }

    pub fn delete_char_before(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }
        let mut new_pos = self.cursor_pos - 1;
        while new_pos > 0 && !self.input.is_char_boundary(new_pos) {
            new_pos -= 1;
        }
        self.input.drain(new_pos..self.cursor_pos);
        self.cursor_pos = new_pos;
    }

    pub fn delete_char_after(&mut self) {
        if self.cursor_pos >= self.input.len() {
            return;
        }
        let mut end = self.cursor_pos + 1;
        while end < self.input.len() && !self.input.is_char_boundary(end) {
            end += 1;
        }
        self.input.drain(self.cursor_pos..end);
    }

    pub fn move_left(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }
        self.cursor_pos -= 1;
        while self.cursor_pos > 0 && !self.input.is_char_boundary(self.cursor_pos) {
            self.cursor_pos -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_pos >= self.input.len() {
            return;
        }
        self.cursor_pos += 1;
        while self.cursor_pos < self.input.len() && !self.input.is_char_boundary(self.cursor_pos) {
            self.cursor_pos += 1;
        }
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_pos = 0;
    }
}

// ASCII LOGO
fn calculator_ascii() -> Vec<&'static str> {
    vec![
        "  ╔═════════════════════════╗  ",
        "  ║  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ║  ",
        "  ║  ▓  ┌──────────────┐ ▓  ║  ",
        "  ║  ▓  │  CALC  TUI   │ ▓  ║  ",
        "  ║  ▓  │    v 1.0     │ ▓  ║  ",
        "  ║  ▓  └──────────────┘ ▓  ║  ",
        "  ║  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ║  ",
        "  ║  ░░░░░░░░░░░░░░░░░░░░░  ║  ",
        "  ║  ░ [7] [8] [9]  [÷]  ░  ║  ",
        "  ║  ░ [4] [5] [6]  [×]  ░  ║  ",
        "  ║  ░ [1] [2] [3]  [-]  ░  ║  ",
        "  ║  ░ [0] [.] [=]  [+]  ░  ║  ",
        "  ║  ░░░░░░░░░░░░░░░░░░░░░  ║  ",
        "  ╚═════════════════════════╝  ",
    ]
}
//ASCII LOGO
//FUNCIONES DE RENDER
fn render_body(f: &mut Frame, area: Rect, app: &App) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(36), Constraint::Min(0)])
        .split(area);

    //CONSTRUIR LOGO CALCULADORA

    let lines: Vec<Line> = calculator_ascii()
        .iter()
        .map(|row| {
            let spans: Vec<Span> = row
                .chars()
                .map(|c| {
                    let style = match c {
                        '▓' => Style::default().fg(Color::Rgb(0, 200, 160)),
                        '░' => Style::default().fg(Color::Rgb(30, 70, 60)),
                        '╔' | '╚' | '╝' | '╗' | '║' => {
                            Style::default().fg(Color::Rgb(0, 200, 160))
                        }
                        '┌' | '└' | '┘' | '┐' | '│' | '─' => {
                            Style::default().fg(Color::Rgb(160, 210, 190))
                        }
                        '[' | ']' => Style::default()
                            .fg(Color::Rgb(0, 220, 150))
                            .add_modifier(Modifier::BOLD),
                        '÷' | '×' | '+' | '-' | '=' | '.' => Style::default()
                            .fg(Color::Rgb(255, 180, 80))
                            .add_modifier(Modifier::BOLD),
                        '0'..='9' => Style::default().fg(Color::Rgb(200, 230, 220)),
                        _ => Style::default().fg(Color::Rgb(80, 110, 100)),
                    };
                    Span::styled(c.to_string(), style)
                })
                .collect();
            Line::from(spans)
        })
        .collect();

    let panel = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(0, 200, 160)))
                .title(Line::from(Span::styled(
                    " ◈ ",
                    Style::default().fg(Color::Rgb(0, 220, 180)),
                )))
                .style(Style::default().bg(Color::Rgb(10, 14, 20)))
                .padding(Padding::new(0, 0, 1, 0)),
        )
        .alignment(Alignment::Center);

    f.render_widget(panel, columns[0]);
    render_right_panel(f, columns[1], app);
}

fn render_right_panel(f: &mut Frame, area: Rect, app: &App) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(5)])
        .split(area);
    render_history(f, rows[0], app);
    render_prompt(f, rows[1], app)
}
fn render_prompt(f: &mut Frame, area: Rect, app: &App) {
    let before = &app.input[..app.cursor_pos];

    // Cursor character (highlighted)
    let (cursor_char, after_cursor) = if app.cursor_pos >= app.input.len() {
        (" ".to_string(), "".to_string())
    } else {
        let end = app.input[app.cursor_pos..]
            .char_indices()
            .nth(1)
            .map(|(i, _)| app.cursor_pos + i)
            .unwrap_or(app.input.len());
        (
            app.input[app.cursor_pos..end].to_string(),
            app.input[end..].to_string(),
        )
    };

    let prompt_line = Line::from(vec![
        Span::styled(
            "  λ ",
            Style::default()
                .fg(Color::Rgb(0, 220, 160))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            before.to_string(),
            Style::default().fg(Color::Rgb(220, 235, 225)),
        ),
        Span::styled(
            cursor_char,
            Style::default()
                .fg(Color::Rgb(10, 14, 20))
                .bg(Color::Rgb(0, 220, 160)),
        ),
        Span::styled(after_cursor, Style::default().fg(Color::Rgb(220, 235, 225))),
    ]);

    let hint_line = Line::from(vec![
        Span::styled(
            "  Enter",
            Style::default()
                .fg(Color::Rgb(0, 200, 140))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" calcular  ", Style::default().fg(Color::Rgb(50, 70, 65))),
        Span::styled(
            "Esc",
            Style::default()
                .fg(Color::Rgb(0, 200, 140))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" limpiar  ", Style::default().fg(Color::Rgb(50, 70, 65))),
        Span::styled(
            "?",
            Style::default()
                .fg(Color::Rgb(0, 200, 140))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ayuda  ", Style::default().fg(Color::Rgb(50, 70, 65))),
        Span::styled(
            "Ctrl+C",
            Style::default()
                .fg(Color::Rgb(0, 200, 140))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" salir", Style::default().fg(Color::Rgb(50, 70, 65))),
    ]);

    let widget = Paragraph::new(vec![prompt_line, hint_line])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(0, 160, 120)))
                .title(Line::from(Span::styled(
                    "",
                    Style::default().fg(Color::Rgb(150, 205, 185)),
                )))
                .style(Style::default().bg(Color::Rgb(8, 14, 16))),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(widget, area);
}
fn render_history(f: &mut Frame, area: Rect, app: &App) {
    let mut items: Vec<ListItem> = Vec::new();

    if app.history.is_empty() {
        items.push(ListItem::new(Line::from("")));
        items.push(ListItem::new(Line::from(vec![
            Span::styled(
                "  ░  Sin historial.  ",
                Style::default().fg(Color::Rgb(45, 60, 55)),
            ),
            Span::styled(
                "Escribe una expresión y pulsa Enter.",
                Style::default().fg(Color::Rgb(65, 85, 75)),
            ),
        ])));
        items.push(ListItem::new(Line::from("")));
        items.push(ListItem::new(Line::from(vec![
            Span::styled("  Ejemplos: ", Style::default().fg(Color::Rgb(55, 75, 65))),
            Span::styled(
                "2+2  •  (3*4)/2  •  2^8  •  3.1415*2",
                Style::default()
                    .fg(Color::Rgb(0, 170, 120))
                    .add_modifier(Modifier::ITALIC),
            ),
        ])));
    } else {
        for (i, entry) in app.history.iter().enumerate() {
            let idx = format!("{:>3}│ ", i + 1);
            let sep = "    └─────────────────────────────────────".to_string();

            let expr_line = Line::from(vec![
                Span::styled(idx, Style::default().fg(Color::Rgb(45, 60, 55))),
                Span::styled(
                    entry.expr.clone(),
                    Style::default().fg(Color::Rgb(160, 200, 185)),
                ),
            ]);

            let result_line = match &entry.result {
                Ok(v) => Line::from(vec![
                    Span::styled("    │   = ", Style::default().fg(Color::Rgb(45, 60, 55))),
                    Span::styled(
                        fmt_result(*v),
                        Style::default()
                            .fg(Color::Rgb(0, 230, 160))
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Err(e) => Line::from(vec![
                    Span::styled("    │ ✗ ", Style::default().fg(Color::Rgb(200, 70, 70))),
                    Span::styled(e.clone(), Style::default().fg(Color::Rgb(210, 110, 110))),
                ]),
            };

            let sep_line = Line::from(Span::styled(
                sep,
                Style::default().fg(Color::Rgb(28, 38, 50)),
            ));

            items.push(ListItem::new(vec![expr_line, result_line, sep_line]));
        }
    }
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(35, 50, 65)))
            .title(Line::from(vec![
                Span::styled(" ◈ ", Style::default().fg(Color::Rgb(0, 180, 220))),
                Span::styled("Historial", Style::default().fg(Color::Rgb(150, 185, 210))),
                Span::styled(
                    format!("  {} op. ", app.history.len()),
                    Style::default().fg(Color::Rgb(45, 65, 85)),
                ),
            ]))
            .style(Style::default().bg(Color::Rgb(10, 13, 20))),
    );
    f.render_widget(list, area);
}
fn render_help(f: &mut Frame, area: Rect) {
    let pw = 56u16.min(area.width);
    let ph = 20u16.min(area.height);
    let popup = ratatui::layout::Rect {
        x: area.x + (area.width.saturating_sub(pw)) / 2,
        y: area.y + (area.height.saturating_sub(ph)) / 2,
        width: pw,
        height: ph,
    };

    let lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Operadores",
            Style::default()
                .fg(Color::Rgb(0, 210, 160))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  +  -  *  /  ^  %",
            Style::default().fg(Color::Rgb(180, 220, 200)),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Ejemplos",
            Style::default()
                .fg(Color::Rgb(0, 210, 160))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  (2+3)*4   2^8",
            Style::default().fg(Color::Rgb(180, 220, 200)),
        )]),
        Line::from(vec![Span::styled(
            "   3.1415*r^2",
            Style::default().fg(Color::Rgb(180, 220, 200)),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Pulsa ", Style::default().fg(Color::Rgb(75, 100, 90))),
            Span::styled(
                "?",
                Style::default()
                    .fg(Color::Rgb(0, 220, 160))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" o ", Style::default().fg(Color::Rgb(75, 100, 90))),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Rgb(0, 220, 160))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" para cerrar", Style::default().fg(Color::Rgb(75, 100, 90))),
        ]),
        Line::from(""),
    ];

    let bg = Block::default().style(Style::default().bg(Color::Rgb(5, 8, 12)));
    f.render_widget(bg, area);

    let help = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Rgb(0, 200, 150)))
            .title(Line::from(vec![
                Span::styled(" ? ", Style::default().fg(Color::Rgb(0, 220, 160))),
                Span::styled(
                    "Ayuda",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ", Style::default()),
            ]))
            .title_alignment(Alignment::Center)
            .style(Style::default().bg(Color::Rgb(8, 16, 14))),
    );
    f.render_widget(help, popup);
}
//FUNCIONES DE RENDER

pub fn render(f: &mut Frame, app: &App) {
    let size = f.size();
    let bg = Block::default().style(Style::default().bg(Color::Rgb(10, 12, 18)));

    f.render_widget(bg, size);
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(size);

    render_body(f, root[1], app);

    if app.show_help {
        render_help(f, size);
    }
}
