pub fn to_url(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == ' ' {
                "%20".to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
    }
}
