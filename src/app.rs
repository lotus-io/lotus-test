use std::io;

use crate::theme::{Theme, discover_themes};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Padding},
    style::{Style},
    widgets::{Block, List, ListItem},
};

struct App {
    should_quit: bool,
    themes: Vec<Theme>,
}

impl App {
    fn theme(&self) -> &Theme {
        self.themes
            .first()
            .expect("at least one theme should always exist")
    }
}

fn quit(app: &mut App) {
    app.should_quit = true;
}

fn handle_shortcut(app: &mut App, c: char) {
    match c.to_ascii_lowercase() {
        'q' => quit(app),
        _ => {}
    }
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let themes = discover_themes();

    let mut app = App {
        should_quit: false,
        themes,
    };

    loop {
        terminal.draw(|frame| draw_home(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            handle_home_input(&mut app, key);
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_home_input(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if let KeyCode::Char(c) = key.code {
            handle_shortcut(app, c);
        }
    } else {
        match key.code {
            KeyCode::Esc => quit(app),
            _ => {}
        }
    }
}

fn draw_home(frame: &mut Frame, app: &mut App) {
    let theme = app.theme();
    let full_area = frame.area();

    frame.render_widget(
        Block::default().style(
            Style::default()
                .bg(theme.colors.base)
                .fg(theme.colors.text),
        ),
        full_area,
    );

    let outer_block = Block::default().padding(Padding::proportional(1));
    let inner_area = outer_block.inner(full_area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(inner_area);

    let header = ratatui::widgets::Paragraph::new(
        " [↑↓] Navigate   [^s] Select   [^l] Load   [Esc] Quit",
    )
    .style(Style::default().fg(theme.colors.accent));

    frame.render_widget(header, vertical[0]);

    let items = theme_color_items(theme);

    let list = List::new(items)
        .block(Block::default())
        .style(
            Style::default()
                .bg(theme.colors.base)
                .fg(theme.colors.text),
        );

    frame.render_widget(list, vertical[1]);
}

const separator = color_item("------", theme.colors.border);

fn theme_color_items(theme: &Theme) -> Vec<ListItem<'static>> {
    vec![
        color_item("text", theme.colors.text),
        color_item("base", theme.colors.base),

        separator,

        color_item("surface_0", theme.colors.surface_0),
        color_item("surface_1", theme.colors.surface_1),

        separator,

        color_item("muted", theme.colors.muted),
        color_item("subtle", theme.colors.subtle),
        color_item("disabled", theme.colors.disabled),

        separator,

        color_item("border", theme.colors.border),

        separator,

        color_item("accent", theme.colors.accent),
        color_item("accent_2", theme.colors.accent_2),
        color_item("accent_3", theme.colors.accent_3),
        color_item("accent_4", theme.colors.accent_4),
        color_item("accent_5", theme.colors.accent_5),

        separator,

        color_item("success", theme.colors.success),
        color_item("warning", theme.colors.warning),
        color_item("error", theme.colors.error),
        color_item("info", theme.colors.info),

        separator,

        color_item("link", theme.colors.link),

        separator,

        color_item("black", theme.colors.black),
        color_item("blue", theme.colors.blue),
        color_item("green", theme.colors.green),
        color_item("cyan", theme.colors.cyan),
        color_item("red", theme.colors.red),
        color_item("magenta", theme.colors.magenta),
        color_item("brown", theme.colors.brown),
        color_item("light_gray", theme.colors.light_gray),
        color_item("dark_gray", theme.colors.dark_gray),
        color_item("light_blue", theme.colors.light_blue),
        color_item("light_green", theme.colors.light_green),
        color_item("light_cyan", theme.colors.light_cyan),
        color_item("light_red", theme.colors.light_red),
        color_item("light_magenta", theme.colors.light_magenta),
        color_item("yellow", theme.colors.yellow),
        color_item("white", theme.colors.white),
    ]
}

fn color_item(name: &'static str, color: ratatui::style::Color) -> ListItem<'static> {
    ListItem::new(format!("█ {name}"))
        .style(Style::default().fg(color))
}
