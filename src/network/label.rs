// Fixed Text

use serde::{Deserialize, Serialize};

use crate::Position;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Label {
    text: [char; 25],
    pos: Position,
}

impl Label {
    pub fn from_str(s: &str) -> Option<Self> {
        let ln = s.len();
        if ln == 0 {
            return None;
        } else if ln > 25 {
            let mut buf = ['\0'; 25];
            for (i, c) in s.chars().take(25).enumerate() {
                buf[i] = c;
            }
            return Some(Label {
                text: buf,
                pos: Position { x: 0.0, y: 0.0 },
            });
        } else {
            let mut buf = ['\0'; 25];
            for (i, c) in s.chars().enumerate() {
                buf[i] = c;
            }
            return Some(Label {
                text: buf,
                pos: Position { x: 0.0, y: 0.0 },
            });
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
        let lstr = labl.unwrap_or_default().as_string();

        let strng = format!("Tank 1000m3 in local area");
        assert_eq!(strng, lstr);
    }

    #[test]
    fn label_less_25_chars() {
        let s = "Tank 1000m3";
        let labl = Label::from_str(s);
        let lstr = labl.unwrap_or_default().as_string();

        let strng = format!("Tank 1000m3");
        assert_eq!(strng, lstr);
    }
}
