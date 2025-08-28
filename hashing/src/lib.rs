pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut v = list.to_vec();
    v.sort();
    if list.len() % 2 == 0 {
        let first_idx = v.len() / 2 - 1;
        let second_idx = v.len() / 2;

        let first_num = v[first_idx];
        let second_num = v[second_idx];

        (first_num + second_num) / 2
    } else {
        let idx = v.len() / 2;
        v[idx]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut max_count = 0;
    let mut mode = list[0];

    for i in 0..list.len() {
        let mut count = 0;
        for j in 0..list.len() {
            if list[i] == list[j] {
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
            mode = list[i];
        }
    }
    mode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = [4, 7, 5, 2, 5, 1, 3];

        println!("mean {}", mean(&v));
        println!("median {}", median(&v));
        println!("mode {}", mode(&v));
    }
}
