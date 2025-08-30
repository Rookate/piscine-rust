pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut total: u64 = arr.iter().sum();
    let mut res = Vec::with_capacity(arr.len());
    for &x in arr.iter().rev() {
        res.push(total);
        total -= x;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "Partial sums of [5, 18, 3, 23] is : {:?}",
            parts_sums(&[5, 18, 3, 23])
        );
    }
}
