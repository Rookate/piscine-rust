pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut at_word_start = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            at_word_start = true;
        } else if at_word_start {
            for uc in c.to_uppercase() {
                result.push(uc);
            }
            at_word_start = false;
        } else {
            result.push(c);
        }
    }

    result
}

// pub fn title_case(input: &str) -> String {
//     input
//         .split_whitespace()
//         .map(|word| capitalize_first(word))
//         .collect::<Vec<String>>()
//         .join(" ")
// }

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .flat_map(|letter| {
            if letter.is_uppercase() {
                letter.to_lowercase().collect::<Vec<_>>()
            } else if letter.is_lowercase() {
                letter.to_uppercase().collect::<Vec<_>>()
            } else {
                vec![letter]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", capitalize_first("joe is missing"));
        println!("{}", title_case("jill is leaving A"));
        println!("{}", change_case("heLLo THere"));
    }
}
