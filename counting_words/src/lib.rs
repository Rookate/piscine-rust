use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut count = HashMap::new();

    for raw in words.split_whitespace() {
        let clean = raw
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();

        if !clean.is_empty() {
            *count.entry(clean).or_insert(0) += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", counting_words("Hello, world!"));
        println!("{:?}", counting_words("“Two things are infinite: the universe and human stupidity; and I'm not sure about the universe.”
    ― Albert Einstein "));
        println!("{:?}", counting_words("Batman, BATMAN, batman, Stop stop"));
    }
}
