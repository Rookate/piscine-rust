use std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let filename = "created.txt";
        File::create(filename).unwrap();

        println!("{:?}", open_file(filename));

        fs::remove_file(filename).unwrap();

        // this should panic!
        open_file(filename);
    }
}
