pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            let mut out = String::new();

            for (i, word) in name.split_whitespace().enumerate() {
                if let Some(ch) = word.chars().next() {
                    if i > 0 {
                        out.push_str(". ");
                    }
                    out.push(ch);
                }
            }

            if !out.is_empty() {
                out.push('.');
            }

            out
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        println!("{:?}", initials(names));
    }
}
