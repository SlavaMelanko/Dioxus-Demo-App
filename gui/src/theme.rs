pub enum Id {
    Light,
    Dark,
}

pub trait Theme {
    fn id(&self) -> Id;

    fn text_light(&self) -> &str;
    fn text_dark(&self) -> &str;

    fn back_light(&self) -> &str;
    fn back_mid(&self) -> &str;
    fn back_dark(&self) -> &str;

    fn img_theme(&self) -> &str;
}

pub struct LightTheme {}

impl LightTheme {
    pub fn new() -> Box<dyn Theme> {
        Box::new(LightTheme {})
    }
}

impl Theme for LightTheme {
    fn id(&self) -> Id {
        Id::Light
    }

    fn text_light(&self) -> &str {
        "#242526"
    }

    fn text_dark(&self) -> &str {
        "#18191a"
    }

    fn back_light(&self) -> &str {
        "#e4e6eb"
    }

    fn back_mid(&self) -> &str {
        "#3a3b3c"
    }

    fn back_dark(&self) -> &str {
        "#b0b3b8"
    }

    fn img_theme(&self) -> &str {
        "gui/img/moon-32.png"
    }
}

pub struct DarkTheme {}

impl DarkTheme {
    pub fn new() -> Box<dyn Theme> {
        Box::new(DarkTheme {})
    }
}

impl Theme for DarkTheme {
    fn id(&self) -> Id {
        Id::Dark
    }

    fn text_light(&self) -> &str {
        "#e4e6eb"
    }

    fn text_dark(&self) -> &str {
        "#b0b3b8"
    }

    fn back_light(&self) -> &str {
        "#242526"
    }

    fn back_mid(&self) -> &str {
        "#3a3b3c"
    }

    fn back_dark(&self) -> &str {
        "#18191a"
    }

    fn img_theme(&self) -> &str {
        "gui/img/sun-32.png"
    }
}

pub fn get_default_theme_provider() -> Box<dyn Theme> {
    Box::new(DarkTheme {})
}
