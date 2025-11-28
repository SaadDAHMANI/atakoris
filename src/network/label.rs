// Fixed Text

use serde::{Deserialize, Serialize};

use crate::Position;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Label {
    text: [char; 25],
    pub pos: Position,
}

impl Label {
    pub fn from_str(s: &str) -> Self {
        let ln = s.len();
        let mut buf = ['\0'; 25];
        if ln > 25 {
            for (i, c) in s.chars().take(25).enumerate() {
                buf[i] = c;
            }
            return Label {
                text: buf,
                pos: Position { x: 0.0, y: 0.0 },
            };
        } else {
            for (i, c) in s.chars().enumerate() {
                buf[i] = c;
            }
            return Label {
                text: buf,
                pos: Position { x: 0.0, y: 0.0 },
            };
        }
    }

    pub fn set_position(mut self, pos: Position) -> Self {
        self.pos = pos;
        self
    }
    pub fn update_by(&mut self, s: &str) {
        for (i, c) in s.chars().enumerate() {
            if i < 25 {
                self.text[i] = c;
            }
        }

        for i in s.len()..25 {
            self.text[i] = '\0';
        }
    }

    pub fn as_string(&self) -> String {
        self.text.iter().take_while(|c| **c != '\0').collect()
    }
}

impl Default for Label {
    fn default() -> Self {
        let buf = ['\0'; 25];

        Self {
            text: buf,
            pos: Position { x: 0.0, y: 0.0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Label;

    #[test]
    fn label_more_25_chars() {
        let s = "Tank 1000m3 in local area at elevation 50m";
        let labl = Label::from_str(s);
        let lstr = labl.as_string();
        let strng = format!("Tank 1000m3 in local area");
        assert_eq!(strng, lstr);
    }

    #[test]
    fn label_less_25_chars() {
        let s = "Tank 1000m3";
        let labl = Label::from_str(s);
        let lstr = labl.as_string();

        let strng = format!("Tank 1000m3");
        assert_eq!(strng, lstr);
    }

    #[test]
    fn label_update_chars() {
        let s = "Tank 1000m3";
        let mut labl = Label::from_str(s);

        labl.update_by("Roua-Lil");

        let strng = format!("Roua-Lil");
        assert_eq!(strng, labl.as_string());
    }

    #[test]
    fn label_update_by_more_25_chars() {
        let s = "12345678901234567890123456789";
        let mut labl = Label::from_str(s);

        labl.update_by("159");

        let strng = format!("159");
        assert_eq!(strng, labl.as_string());
    }
}
