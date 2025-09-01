use std::fs::OpenOptions;
use std::{io::Write, path::Path};

pub fn open_or_create<P: AsRef<Path>>(path: P, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
        .expect("Unable to open file");

    file.write_all(content.as_bytes())
        .expect("Unable to write data");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let path = "a.txt";

        open_or_create(&path, "content to be written");

        let contents = fs::read_to_string(path).unwrap();
        println!("{}", contents);
    }
}
