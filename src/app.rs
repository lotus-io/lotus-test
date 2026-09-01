use std::io;

use crate::theme::{Theme, discover_themes, find_theme_index};
use crate::ui::home::draw_home;
use crate::input::home::handle_home_input;

use crossterm::event::{self, Event, KeyEvent};
use ratatui::{Terminal, backend::CrosstermBackend, widgets::ListState};

struct App {
    should_quit: bool,
    themes: Vec<Theme>,
}

impl App {
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
        terminal.draw(|frame| draw_home(frame, &mut app));

        if let Event::Key(key) = event::read()? {
            app.last_key_event = Some(key);

            handle_home_input(&mut app, key)
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
    let theme = app.theme().clone();
    let full_area = frame.area();

    frame.render_widget(
        Block::default().style(
            Style::default()
                .bg(theme.colors.background)
                .fg(theme.colors.text),
        ),
        full_area,
    );

    let outer_block = Block::default().padding(Padding::proportional(1));

    let inner_area = outer_block.inner(full_area);

    let constraints = vec![
        Constraint::Fill(1),
    ];

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    let text = Paragraph::new(" [↑↓]  [^s]  [^l]").style(Style::default().fg(theme.colors.accent));

    frame.render_widget(text, vertical[0]);
}



// "█ text" , "#CDD6F4";
// "█ base" , "#1E1E2E";
// "█ surface_0" , "#313244";
// "█ surface_1" , "#45475A";
// "█ muted" , "#6C7086";
// "█ subtle" , "#585B70";
// "█ disabled" , "#6C7086";
// "█ border" , "#6C7086";
// "█ accent" , "#CBA6F7";
// "█ accent_2" , "#89B4FA";
// "█ accent_3" , "#FAB387";
// "█ accent_4" , "#A6E3A1";
// "█ accent_5" , "#F5C2E7";
// "█ success" , "#A6E3A1";
// "█ warning" , "#F9E2AF";
// "█ error" , "#F38BA8";
// "█ info" , "#89DCEB";
// "█ link" , "#89B4FA";
// "█ black" , "#1E1E2E"
// "█ blue" , "#667FA8"
// "█ green" , "#769C72"
// "█ cyan" , "#6D9E99"
// "█ red" , "#B4546A"
// "█ magenta" , "#9B78B8"
// "█ brown" , "#B39A68"
// "█ light_gray" , "#A6ADC8"
// "█ dark_gray" , "#6C7086"
// "█ light_blue" , "#89B4FA"
// "█ light_green" , "#A6E3A1"
// "█ light_cyan" , "#94E2D5"
// "█ light_red" , "#F38BA8"
// "█ light_magenta" , "#CBA6F7"
// "█ yellow" , "#F9E2AF"
// "█ white" , "#CDD6F4"