pub fn first_subword(mut s: String) -> String {
    let index = match s.chars().skip(1).position(|c| c.is_uppercase() || c == '_') {
        Some(idx) => idx + 1,
        None => return s,
    };
    s.truncate(index);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "helloWorld";
        let s2 = "snake_case";
        let s3 = "CamelCase";
        let s4 = "just";

        println!("first_subword({}) = {}", s1, first_subword(s1.to_owned()));
        println!("first_subword({}) = {}", s2, first_subword(s2.to_owned()));
        println!("first_subword({}) = {}", s3, first_subword(s3.to_owned()));
        println!("first_subword({}) = {}", s4, first_subword(s4.to_owned()));
    }
}
