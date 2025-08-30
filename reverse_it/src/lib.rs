pub fn reverse_it(v: i32) -> String {
    if v < 0 {
        let s = (-v).to_string();
        format!("-{}{}", s.chars().rev().collect::<String>(), s)
    } else {
        let s = v.to_string();
        format!("{}{}", s.chars().rev().collect::<String>(), s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", reverse_it(123));
        println!("{}", reverse_it(-123));
    }
}
