pub fn delete_and_backspace(s: &mut String) {
    let mut out = String::new();
    let mut skip_letters = 0;
    let mut it = s.chars();

    while let Some(ch) = it.next() {
        match ch {
            '+' => skip_letters += 1,
            '-' => {
                let _ = out.pop();
            }
            _ => {
                if skip_letters > 0 {
                    skip_letters -= 1;
                } else {
                    out.push(ch);
                }
            }
        }
    }

    *s = out;
}

pub fn do_operations(v: &mut [String]) {
    for expr in v.iter_mut() {
        let (idx, op) = expr
            .char_indices()
            .find(|&(_, c)| c == '+' || c == '-')
            .expect("no operator found");

        let (left_s, right_s) = expr.split_at(idx);

        let left: i32 = left_s.parse().unwrap();
        let right: i32 = right_s[op.len_utf8()..].parse().unwrap();

        let value = match op {
            '+' => left + right,
            '-' => left - right,
            _ => unreachable!(),
        };
        *expr = value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        let mut b = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "5+5".to_owned(),
        ];

        delete_and_backspace(&mut a);
        do_operations(&mut b);

        println!("{:?}", (a, b));
    }
}
