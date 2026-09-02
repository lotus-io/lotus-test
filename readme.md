# Rust

In this scenario we are assuming you have a folder structure that looks like this:

```bash
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── app.rs
│   ├── main.rs
│   └── theme.rs
```

The theme format provides both semantic colors and a complete ANSI-style palette. You generally do not need to use every color in your application.

For most TUI applications, semantic colors such as `text`, `base`, `border`, `accent`, `success`, `warning`, `error`, and `info` are enough. The additional accent and ANSI palette colors are useful when an application needs more visual variety, such as for syntax highlighting, charts, illustrations, or other multi-color UI elements.

## Getting themes

In order to install all the themes into `~/.config/lotus/`:

```bash
mkdir -p ~/.config/lotus
cd ~/.config/lotus
git clone --filter=blob:none --no-checkout https://github.com/lotus-io/lotus.git .
git sparse-checkout init --no-cone
git sparse-checkout set 'themes/**'
git checkout
rm -rf .git
```

## Rust

In order to be able to interact with the themes I suggest having something like `src/themes.rs` with the folowwing helpers:

```rust
use ratatui::style::Color;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct Theme {
    pub name: String,
    pub author: String,
    pub colors: ThemeColors,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemeColors {
    #[serde(deserialize_with = "deserialize_color")]
    pub text: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub base: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub surface_0: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub surface_1: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub muted: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub subtle: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub disabled: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub border: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub accent: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub accent_2: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub accent_3: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub accent_4: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub accent_5: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub success: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub warning: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub error: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub info: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub link: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub black: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub blue: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub green: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub cyan: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub red: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub magenta: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub brown: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub light_gray: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub dark_gray: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub light_blue: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub light_green: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub light_cyan: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub light_red: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub light_magenta: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub yellow: Color,

    #[serde(deserialize_with = "deserialize_color")]
    pub white: Color,
}

impl Theme {
    pub fn fallback() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            author: env!("CARGO_PKG_AUTHORS").to_string(),
            description: "Built-in fallback theme.".to_string(),
            colors: ThemeColors {
                // Base
                text: Color::White,
                base: Color::Black,

                // Surfaces
                surface_0: Color::DarkGray,
                surface_1: Color::Gray,

                // Text
                muted: Color::DarkGray,
                subtle: Color::DarkGray,
                disabled: Color::DarkGray,

                // UI
                border: Color::DarkGray,

                // Accents
                accent: Color::Magenta,
                accent_2: Color::Blue,
                accent_3: Color::Yellow,
                accent_4: Color::Green,
                accent_5: Color::Magenta,

                // Semantic
                success: Color::Green,
                warning: Color::Yellow,
                error: Color::Red,
                info: Color::Cyan,

                // Links
                link: Color::Blue,

                // ANSI palette
                black: Color::Black,
                blue: Color::Blue,
                green: Color::Green,
                cyan: Color::Cyan,
                red: Color::Red,
                magenta: Color::Magenta,
                brown: Color::Yellow,
                light_gray: Color::Gray,
                dark_gray: Color::DarkGray,
                light_blue: Color::LightBlue,
                light_green: Color::LightGreen,
                light_cyan: Color::LightCyan,
                light_red: Color::LightRed,
                light_magenta: Color::LightMagenta,
                yellow: Color::Yellow,
                white: Color::White,
            },
        }
    }
}

pub fn parse_hex_color(raw: &str) -> Option<Color> {
    let hex = raw.trim().trim_start_matches('#');

    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color::Rgb(r, g, b))
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;

    parse_hex_color(&raw)
        .ok_or_else(|| serde::de::Error::custom(format!("invalid hex color: {raw:?}")))
}

fn toml_theme_parser(raw: &str) -> Result<Theme, toml::de::Error> {
    toml::from_str(raw)
}

fn config_dir() -> Option<std::path::PathBuf> {
    let home = std::env::var_os("HOME")?;

    Some(
        std::path::PathBuf::from(home)
            .join(".config")
            .join("lotus"),
    )
}

fn themes_dir() -> Option<std::path::PathBuf> {
    Some(config_dir()?.join("themes"))
}

fn current_theme_path() -> Option<std::path::PathBuf> {
    Some(config_dir()?.join("current-theme"))
}


fn load_themes() -> Vec<Theme> {
    let Some(dir) = themes_dir() else {
        return Vec::new();
    };

    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };

    entries
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|ext| ext == "toml")
        })
        .filter_map(|entry| {
            let path = entry.path();

            match std::fs::read_to_string(&path) {
                Ok(raw) => match toml_theme_parser(&raw) {
                    Ok(theme) => Some(theme),
                    Err(err) => {
                        eprintln!(
                            "Failed to parse theme {}: {err}",
                            path.display()
                        );
                        None
                    }
                },
                Err(err) => {
                    eprintln!(
                        "Failed to read theme {}: {err}",
                        path.display()
                    );
                    None
                }
            }
        })
        .collect()
}

pub fn discover_themes() -> Vec<Theme> {
    let mut themes = load_themes();

    themes.sort_by(|a, b| a.name.cmp(&b.name));

    if themes.is_empty() {
        themes.push(Theme::fallback());
    }

    themes
}

pub fn list_themes() {
    let themes = discover_themes();

    for theme in themes {
        println!("{} — {}", theme.name, theme.author);
    }
}

pub fn get_current_theme() -> Theme {
    let themes = discover_themes();

    let Some(path) = current_theme_path() else {
        return themes
            .first()
            .cloned()
            .unwrap_or_else(Theme::fallback);
    };

    let Ok(name) = std::fs::read_to_string(&path) else {
        return themes
            .first()
            .cloned()
            .unwrap_or_else(Theme::fallback);
    };

    let name = name.trim();

    themes
        .into_iter()
        .find(|theme| theme.name == name)
        .unwrap_or_else(Theme::fallback)
}

pub fn set_current_theme(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.trim();

    if name.is_empty() {
        return Err("theme name cannot be empty".into());
    }

    let themes = discover_themes();

    let theme_exists = themes.iter().any(|theme| theme.name == name);

    if !theme_exists {
        return Err(format!("theme not found: {name}").into());
    }

    let config_dir = config_dir()
        .ok_or("could not determine config directory")?;

    std::fs::create_dir_all(&config_dir)?;

    let path = config_dir.join("current-theme");

    std::fs::write(path, format!("{name}\n"))?;

    Ok(())
}
```

then you would just add those in your `src/app.rs`