use crate::password_generator::chars;


pub struct Config {
    pub enable_digit: bool,
    pub enable_lower: bool,
    pub enable_upper: bool,
    pub enable_special: bool,
    pub pass_lenght: u64,
}

impl Config {
    pub fn new(&self) -> Vec<char> {
        let mut result = vec![];

        if self.enable_digit {
            for i in chars::NUMS {
                result.push(i);
            }
        }

        if self.enable_lower {
            for i in chars::LETTERS_LOWER {
                result.push(i);
            }
        }

        if self.enable_upper {
            for i in chars::LETTERS_UPPER {
                result.push(i);
            }
        }

        if self.enable_special {
            for i in chars::SPECIAL_CHARS {
                result.push(i);
            }
        }

        result
    }
}
