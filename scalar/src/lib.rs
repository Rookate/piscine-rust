pub fn sum(a: u8, b: u8) -> u8 {
    return a + b;
}

pub fn diff(a: i16, b: i16) -> i16 {
    return a - b;
}

pub fn pro(a: i8, b: i8) -> i8 {
    return a * b;
}

pub fn quo(a: f32, b: f32) -> f32 {
    return a / b;
}

pub fn rem(a: f32, b: f32) -> f32 {
    return a % b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // sum
        println!("sum: {}", sum(234, 2)); // 'sum: 236'

        // diff
        println!("diff: {}", diff(234, 2)); // 'diff: 232'

        // product
        println!("pro: {}", pro(23, 2)); // 'pro: 46'

        // quotient
        println!("quo: {}", quo(22.0, 2.0)); // 'quo: 11'
        println!("quo: {}", quo(-128.23, 2.0)); // 'quo: -64.115'

        // remainder
        println!("rem: {}", rem(22.0, 2.0)); // 'rem: 0'
    }
}
