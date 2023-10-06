#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Id {
    Light,
    Dark,
}

pub struct FontColorPalette {
    pub light: &'static str,
    pub dark: &'static str,
}

pub struct BackgroundColorPalette {
    pub light: &'static str,
    pub mid: &'static str,
    pub dark: &'static str,
}

pub struct ImageProvider {
    pub theme: &'static str,
    pub quit: &'static str,
    pub settings: &'static str,
    pub about: &'static str,
}

pub struct ThemeConfig {
    pub id: Id,
    pub font: FontColorPalette,
    pub back: BackgroundColorPalette,
    pub img: ImageProvider,
}

/*
    https://www.color-hex.com/color-palette/99155
*/
impl ThemeConfig {
    pub fn make_light_theme_config() -> Self {
        ThemeConfig {
            id: Id::Light,
            font: FontColorPalette {
                light: "#242526",
                dark: "#18191a",
            },
            back: BackgroundColorPalette {
                light: "#e4e6eb",
                mid: "#3a3b3c",
                dark: "#b0b3b8",
            },
            img: ImageProvider {
                theme: "ui/img/theme-l64.png",
                quit: "ui/img/quit-l32.png",
                settings: "ui/img/settings-l32.png",
                about: "ui/img/info-l32.png",
            },
        }
    }

    pub fn make_dark_theme_config() -> Self {
        ThemeConfig {
            id: Id::Dark,
            font: FontColorPalette {
                light: "#e4e6eb",
                dark: "#b0b3b8",
            },
            back: BackgroundColorPalette {
                light: "#242526",
                mid: "#3a3b3c",
                dark: "#18191a",
            },
            img: ImageProvider {
                theme: "ui/img/theme-d64.png",
                quit: "ui/img/quit-d32.png",
                settings: "ui/img/settings-d32.png",
                about: "ui/img/info-d32.png",
            },
        }
    }
}

pub fn get_default_theme() -> ThemeConfig {
    ThemeConfig::make_dark_theme_config()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme_config() {
        let theme = ThemeConfig::make_dark_theme_config();

        assert_eq!(theme.font.light, "#e4e6eb");
        assert_eq!(theme.font.dark, "#b0b3b8");

        assert_eq!(theme.back.light, "#242526");
        assert_eq!(theme.back.mid, "#3a3b3c");
        assert_eq!(theme.back.dark, "#18191a");

        assert_eq!(theme.img.theme, "ui/img/theme-d64.png");
        assert_eq!(theme.img.quit, "ui/img/quit-d32.png");
        assert_eq!(theme.img.settings, "ui/img/settings-d32.png");
        assert_eq!(theme.img.about, "ui/img/info-d32.png");
    }

    #[test]
    fn test_light_theme_config() {
        let theme = ThemeConfig::make_light_theme_config();

        assert_eq!(theme.font.light, "#242526");
        assert_eq!(theme.font.dark, "#18191a");

        assert_eq!(theme.back.light, "#e4e6eb");
        assert_eq!(theme.back.mid, "#3a3b3c");
        assert_eq!(theme.back.dark, "#b0b3b8");

        assert_eq!(theme.img.theme, "ui/img/theme-l64.png");
        assert_eq!(theme.img.quit, "ui/img/quit-l32.png");
        assert_eq!(theme.img.settings, "ui/img/settings-l32.png");
        assert_eq!(theme.img.about, "ui/img/info-l32.png");
    }
}
