use std::io;

use crate::theme::{self, Theme};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Padding},
};

struct App {
    should_quit: bool,
    themes: Vec<Theme>,
    current_theme_index: usize,
    status: Option<String>,

    show_theme_popup: bool,
    popup_state: ListState,
}

impl App {
    fn new(themes: Vec<Theme>) -> Self {
        let mut popup_state = ListState::default();
        popup_state.select(Some(0));

        Self {
            should_quit: false,
            themes,
            current_theme_index: 0,
            status: None,
            show_theme_popup: false,
            popup_state,
        }
    }

    fn theme(&self) -> &Theme {
        self.themes
            .get(self.current_theme_index)
            .or_else(|| self.themes.first())
            .expect("at least one theme should always exist")
    }

    fn open_theme_popup(&mut self) {
        self.show_theme_popup = true;
        self.popup_state.select(Some(self.current_theme_index));
    }

    fn close_theme_popup(&mut self) {
        self.show_theme_popup = false;
    }

    fn popup_move(&mut self, delta: isize) {
        let len = self.themes.len();
        if len == 0 {
            return;
        }

        let current = self.popup_state.selected().unwrap_or(0) as isize;
        let next = (current + delta).rem_euclid(len as isize) as usize;

        self.popup_state.select(Some(next));
    }

    fn apply_selected_theme(&mut self) {
        let Some(index) = self.popup_state.selected() else {
            return;
        };

        let Some(name) = self.themes.get(index).map(|t| t.name.clone()) else {
            return;
        };

        self.current_theme_index = index;

        match theme::set_current_theme(&name) {
            Ok(()) => self.status = Some(format!("Theme set to {name}")),
            Err(err) => self.status = Some(format!("Could not save theme: {err}")),
        }

        self.close_theme_popup();
    }
}

fn quit(app: &mut App) {
    app.should_quit = true;
}

fn handle_shortcut(app: &mut App, c: char) {
    match c.to_ascii_lowercase() {
        'q' => quit(app),
        's' => {
            if app.show_theme_popup {
                app.close_theme_popup();
            } else {
                app.open_theme_popup();
            }
        }
        _ => {}
    }
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let themes = theme::discover_themes();

    let mut app = App::new(themes);

    loop {
        terminal.draw(|frame| draw_home(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            handle_input(&mut app, key);
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn handle_input(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if let KeyCode::Char(c) = key.code {
            handle_shortcut(app, c);
        }
        return;
    }

    if app.show_theme_popup {
        match key.code {
            KeyCode::Esc => app.close_theme_popup(),
            KeyCode::Up | KeyCode::Char('k') => app.popup_move(-1),
            KeyCode::Down | KeyCode::Char('j') => app.popup_move(1),
            KeyCode::Enter => app.apply_selected_theme(),
            _ => {}
        }
    } else {
        match key.code {
            KeyCode::Esc => quit(app),
            _ => {}
        }
    }
}

fn draw_home(frame: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
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
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(inner_area);

    let items = theme_color_items(&theme);

    let list = List::new(items).block(Block::default()).style(
        Style::default()
            .bg(theme.colors.base)
            .fg(theme.colors.text),
    );

    frame.render_widget(list, vertical[0]);

    let help = Paragraph::new(format!(
        "[↑↓] Navigate  [^s] Themes  [Esc] Quit  (theme: {})",
        theme.name
    ))
    .style(Style::default().fg(theme.colors.subtle));

    frame.render_widget(help, vertical[2]);

    if app.show_theme_popup {
        draw_theme_popup(frame, app, &theme, full_area);
    }
}

fn draw_theme_popup(frame: &mut Frame, app: &mut App, theme: &Theme, area: Rect) {
    let popup_area = centered_rect(50, 60, area);

    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Select Theme ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.colors.accent))
        .style(
            Style::default()
                .bg(theme.colors.base)
                .fg(theme.colors.text),
        );

    let items: Vec<ListItem> = app
        .themes
        .iter()
        .enumerate()
        .map(|(index, t)| {
            let marker = if index == app.current_theme_index {
                "● "
            } else {
                "  "
            };

            let mut lines = vec![format!("{marker}{}", t.name)];

            ListItem::new(lines.join("\n"))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(theme.colors.accent)
                .fg(theme.colors.base),
        )
        .highlight_symbol("➤ ");

    frame.render_stateful_widget(list, popup_area, &mut app.popup_state);
}

/// Returns a rect centered within `area`, sized as a percentage of it.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn theme_color_items(theme: &Theme) -> Vec<ListItem<'static>> {
    vec![
        separator(),
        color_item("text", theme.colors.text),
        color_item("base", theme.colors.base),
        separator(),
        color_item("surface_0", theme.colors.surface_0),
        color_item("surface_1", theme.colors.surface_1),
        separator(),
        color_item("muted", theme.colors.muted),
        color_item("subtle", theme.colors.subtle),
        color_item("disabled", theme.colors.disabled),
        separator(),
        color_item("border", theme.colors.border),
        separator(),
        color_item("accent", theme.colors.accent),
        color_item("accent_2", theme.colors.accent_2),
        color_item("accent_3", theme.colors.accent_3),
        color_item("accent_4", theme.colors.accent_4),
        color_item("accent_5", theme.colors.accent_5),
        separator(),
        color_item("success", theme.colors.success),
        color_item("warning", theme.colors.warning),
        color_item("error", theme.colors.error),
        color_item("info", theme.colors.info),
        separator(),
        color_item("link", theme.colors.link),
        separator(),
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
        separator(),
    ]
}

fn separator() -> ListItem<'static> {
    ListItem::new("------------------------").style(Style::default().fg(Color::DarkGray))
}

fn color_item(name: &'static str, color: Color) -> ListItem<'static> {
    ListItem::new(format!("██ {name}")).style(Style::default().fg(color))
}
