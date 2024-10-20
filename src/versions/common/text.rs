use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
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

        // <https://www.digminecraft.com/lists/color_list_pc.php>
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


    fn get_code(&self) -> String {
        match self {
            Self::DarkRed => "4",
            Self::Red => "c",
            Self::Gold => "6",
            Self::Yellow => "e",
            Self::DarkGreen => "2",
            Self::Green => "a",
            Self::Aqua => "b",
            Self::DarkAqua => "3",
            Self::DarkBlue => "1",
            Self::Blue => "9",
            Self::LightPurple => "d",
            Self::DarkPurple => "5",
            Self::White => "f",
            Self::Gray => "7",
            Self::DarkGray => "8",
            Self::Black => "0"
        }.to_string()
    }

    pub fn to_code(&self) -> String {
        format!("§{}", self.get_code())
    }

    pub fn to_code_amperstand(&self) -> String {
        format!("&{}", self.get_code())
    }
}


// <https://minecraft.wiki/w/Raw_JSON_text_format>
// <https://wiki.vg/Text_formatting#Text_components>
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
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
    pub extra: Option<Vec<Self>>,
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
            extra: None,
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

    pub fn to_legacy(&self) -> String {
        let mut result = String::new();
        
        if !self.color.is_none() {
            let color = self.color.unwrap();
            result.push_str(&color.to_code());
        }
        
        if !self.bold.is_none() && self.bold.unwrap() {
            result.push_str("§l");
        }

        if !self.italic.is_none() && self.italic.unwrap() {
            result.push_str("§o");
        }

        if !self.obfuscated.is_none() && self.obfuscated.unwrap() {
            result.push_str("§k");
        }

        if !self.strikethrough.is_none() && self.strikethrough.unwrap() {
            result.push_str("§m");
        }

        if !self.underlined.is_none() && self.underlined.unwrap() {
            result.push_str("§n");
        }

        result.push_str(&self.text);

        if !self.extra.is_none() {
            for mut ext in self.extra.clone().unwrap() {

                {
                    if ext.bold.is_none() && !self.bold.is_none() {
                        ext.bold(self.bold.unwrap());
                    }

                    if ext.italic.is_none() && !self.italic.is_none() {
                        ext.italic(self.italic.unwrap());
                    }

                    if ext.underlined.is_none() && !self.underlined.is_none() {
                        ext.underlined(self.underlined.unwrap());
                    }

                    if ext.strikethrough.is_none() && !self.strikethrough.is_none() {
                        ext.strikethrough(self.strikethrough.unwrap());
                    }

                    if ext.obfuscated.is_none() && !self.obfuscated.is_none() {
                        ext.obfuscated(self.obfuscated.unwrap());
                    }
                }

                result.push_str(&ext.to_legacy());
            }
        }

        result
    }

    pub fn to_legacy_amperstand(&self) -> String {
        let mut result = String::new();
        
        if !self.color.is_none() {
            let color = self.color.unwrap();
            result.push_str(&color.to_code_amperstand());
        }
        
        if !self.bold.is_none() && self.bold.unwrap() {
            result.push_str("&l");
        }

        if !self.italic.is_none() && self.italic.unwrap() {
            result.push_str("&o");
        }

        if !self.obfuscated.is_none() && self.obfuscated.unwrap() {
            result.push_str("&k");
        }

        if !self.strikethrough.is_none() && self.strikethrough.unwrap() {
            result.push_str("&m");
        }

        if !self.underlined.is_none() && self.underlined.unwrap() {
            result.push_str("&n");
        }

        result.push_str(&self.text);

        if !self.extra.is_none() {
            for mut ext in self.extra.clone().unwrap() {

                {
                    if ext.bold.is_none() && !self.bold.is_none() {
                        ext.bold(self.bold.unwrap());
                    }

                    if ext.italic.is_none() && !self.italic.is_none() {
                        ext.italic(self.italic.unwrap());
                    }

                    if ext.underlined.is_none() && !self.underlined.is_none() {
                        ext.underlined(self.underlined.unwrap());
                    }

                    if ext.strikethrough.is_none() && !self.strikethrough.is_none() {
                        ext.strikethrough(self.strikethrough.unwrap());
                    }

                    if ext.obfuscated.is_none() && !self.obfuscated.is_none() {
                        ext.obfuscated(self.obfuscated.unwrap());
                    }
                }

                result.push_str(&ext.to_legacy_amperstand());
            }
        }

        result
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


#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct TextComponent(Vec<StyledText>);

impl TextComponent {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn simple(text: String) -> Self {
        let text = StyledText::new(text);
        let mut text_list = Vec::new();
        text_list.push(text);
        Self(text_list)
    }

    pub fn add(&mut self, text: StyledText) {
        self.0.push(text);
    }

    pub fn to_legacy(&self) -> String {
        let mut res = String::new();

        for text in self.0.clone() {
            res.push_str(&text.to_legacy());
        }

        res
    }

    pub fn to_legacy_amperstand(&self) -> String {
        let mut res = String::new();

        for text in self.0.clone() {
            res.push_str(&text.to_legacy_amperstand());
        }

        res
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().to_string()
    }
}

impl From<StyledText> for TextComponent {
    fn from(value: StyledText) -> Self {
        let mut list = Vec::new();
        list.push(value);

        Self(list)
    }
}

#[cfg(test)]
mod tests {
    use super::{StyledText, TextComponent};


    #[test]
    fn test_legacy() {
        let mut component = TextComponent::new();
        let mut st = StyledText::new("Hi!".to_string());
        st.bold(true);
        st.underlined(true);
        st.color(crate::common::text::StyledTextColor::Gold);
        component.add(st);

        assert_eq!(component.to_legacy(), "§6§l§nHi!");
    }

    #[test]
    fn test_legacy_amperstand() {
        let mut component = TextComponent::new();
        let mut st = StyledText::new("Hi!".to_string());
        st.bold(true);
        st.underlined(true);
        st.color(crate::common::text::StyledTextColor::Gold);
        component.add(st);

        assert_eq!(component.to_legacy_amperstand(), "&6&l&nHi!");
    }
}