pub fn lastup(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let s = word.to_lowercase();
            let mut chars: Vec<char> = s.chars().collect();

            for c in chars.iter_mut().rev() {
                if c.is_alphabetic() {
                    *c = c.to_uppercase().next().unwrap();
                    break;
                }
            }

            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", lastup("joe is missing"));
        println!("{}", lastup("jill is leaving A"));
        println!("{}", lastup("heLLo THere!"));
    }
}
