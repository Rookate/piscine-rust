use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        let mut single_letter = name.to_string();
        single_letter.truncate(1);
        let short_hand = format!("-{single_letter}");
        let long_hand = format!("--{name}");

        Self {
            short_hand,
            long_hand,
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let cb = self
            .flags
            .get(input)
            .ok_or_else(|| format!("Unknown flag: {}", input))?;

        if argv.len() != 2 {
            return Err(format!("Expected 2 arguments, got {}", argv.len()));
        }

        cb(argv[0], argv[1]).map_err(|e| e.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.parse()?;
    Ok((x / y).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.parse()?;
    Ok((x % y).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut handler = FlagsHandler {
            flags: HashMap::new(),
        };

        let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
        let r = Flag::opt_flag(
            "remainder",
            "remainder of the division between two values, formula (a % b)",
        );

        handler.add_flag(d, div);
        handler.add_flag(r, rem);

        println!("{:?}", handler.exec_func("-d", &["1.0", "2.0"]));

        println!("{:?}", handler.exec_func("-r", &["2.0", "2.0"]));

        println!("{:?}", handler.exec_func("--division", &["a", "2.0"]));

        println!("{:?}", handler.exec_func("--remainder", &["2.0", "fd"]));
    }
}
