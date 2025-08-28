pub fn bubble_sort(arr: &mut [i32]) {
    arr.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = [3, 2, 4, 5, 1, 7];
        let mut v_clone = v;

        bubble_sort(&mut v);
        println!("{:?}", v);

        v_clone.sort_unstable();
        println!("{:?}", v_clone);
    }
}
