use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum StyledTextColor {
    DarkRed,
    Red,
    Gold,
    Yellow,
    DarkGreen,
    Green,
    Aqua,
    DarkAqua,
    DarkBlue,
    Blue,
    LightPurple,
    DarkPurple,
    White,
    Gray,
    DarkGray,
    Black
}


impl StyledTextColor {
    pub fn from_code(code: String) -> Self {
        let code = code.to_ascii_lowercase();

        fn check(code: String) -> String {
            if code.starts_with('&') {
                code[1..].to_string()
            } else if code.starts_with('§') {
                code[1..].to_string()
            } else {
                code
            }
        }

        let code = check(code);

        let code = code.as_str();

        // https://www.digminecraft.com/lists/color_list_pc.php
        match code {
            "4" => Self::DarkRed,
            "c" => Self::Red,
            "6" => Self::Gold,
            "e" => Self::Yellow,
            "2" => Self::DarkGreen,
            "a" => Self::Green,
            "b" => Self::Aqua,
            "3" => Self::DarkAqua,
            "1" => Self::DarkBlue,
            "9" => Self::Blue,
            "d" => Self::LightPurple,
            "5" => Self::DarkPurple,
            "f" => Self::White,
            "7" => Self::Gray,
            "8" => Self::DarkGray,
            "0" => Self::Black,
            _ => Self::White
        }
    }
}


// https://minecraft.wiki/w/Raw_JSON_text_forma
// https://wiki.vg/Text_formatting#Text_components
#[derive(Debug, Serialize, Clone)]
pub struct StyledText {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<StyledTextColor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<Self>>
}

impl Default for StyledText {
    fn default() -> Self {
        Self {
            text: "Text".to_string(),
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            extra: None
        }
    }
}

impl StyledText {
    pub fn new(text: String) -> Self {
        Self {
            text: text,
            ..Default::default()
        }
    }

    
    pub fn add_extra(&mut self, text: StyledText) {
        if let Some(extra_vec) = &mut self.extra {
            extra_vec.push(text);
        } else {
            self.extra = Some(vec![text]);
        }
    }


    pub fn color(&mut self, color: StyledTextColor) {
        self.color = Option::from(color);
    }

    pub fn set_color(&mut self, color: StyledTextColor) {
        self.color(color);
    }


    pub fn bold(&mut self, value: bool) {
        self.bold = Option::from(value);
    }

    pub fn italic(&mut self, value: bool) {
        self.italic = Option::from(value);
    }

    pub fn underlined(&mut self, value: bool) {
        self.underlined = Option::from(value);
    }

    pub fn strikethrough(&mut self, value: bool) {
        self.strikethrough = Option::from(value);
    }

    pub fn obfuscated(&mut self, value: bool) {
        self.obfuscated = Option::from(value);
    }
}


#[derive(Debug, Serialize, Clone)]
pub struct TextComponent(Vec<StyledText>);

impl TextComponent {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, text: StyledText) {
        self.0.push(text);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().to_string()
    }
}