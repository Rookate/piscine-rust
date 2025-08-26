pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let x = c as f64;
    let exp_val = x.exp();

    let ln_val = if x == 0.0 {
        f64::NEG_INFINITY
    } else {
        x.abs().ln()
    };

    (c, exp_val, ln_val)
}

pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();

    for char in a.chars() {
        if char == ' ' {
            result.push(' ');
            continue;
        }
        if let Some(current_char) = char.to_digit(10) {
            let y = current_char as f64;
            let exp_val = y.exp();
            result.push_str(&exp_val.to_string());
        }
    }

    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let modified: Vec<f64> = b.iter().map(|&token| (token as f64).abs().ln()).collect();

    (b, modified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "1 2 4 5 6".to_owned();
        let b = vec![1, 2, 4];
        let c = 0;

        println!("{:?}", nbr_function(c));
        println!("{:?}", vec_function(b));
        println!("{:?}", str_function(a));
    }
}
