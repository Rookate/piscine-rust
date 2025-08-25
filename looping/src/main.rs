use std::io;

fn main() {
    let response: &'static str = "The letter e";
    let mut count: i32 = 0;

    loop {
        count = count + 1;
        println!(
            "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
        );
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if response == input.trim() {
                    println!("Number of trials: {}", count);
                    break;
                }
            }
            Err(error) => println!("error: {error}"),
        }
    }
}
