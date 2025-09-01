// pub fn check_ms(message: &str) -> Result<&str, &str> {
//     match message {
//         m if m.is_empty() => Err("ERROR: illegal"),
//         m if m.contains("stupid") => Err("ERROR: illegal"),
//         m => Ok(m),
//     }
// }

pub fn check_ms(message: &str) -> Result<&str, &str> {
    match (message.is_empty(), message.contains("stupid")) {
        (true, _) | (_, true) => Err("ERROR: illegal"),
        _ => Ok(message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        ["hello there", "", "you are stupid", "stupid"]
            .into_iter()
            .for_each(|m| println!("{:?}", check_ms(m)));
    }
}
