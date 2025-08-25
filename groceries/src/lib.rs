pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    slice[index].as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut groceries = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ];
        insert(&mut groceries, String::from("nuts"));
        println!("groceries = {:?}", &groceries);
        println!("groceries[1] = {:?}", at_index(&groceries, 1));
    }
}
