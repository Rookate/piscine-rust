pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "The element in the position {} in fibonacci series is {}",
            2,
            fibonacci(2)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            4,
            fibonacci(4)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            22,
            fibonacci(22)
        );
        println!(
            "The element in the position {} in fibonacci series is {}",
            20,
            fibonacci(20)
        );
    }
}
