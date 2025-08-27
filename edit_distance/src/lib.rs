pub fn edit_distance(source: &str, target: &str) -> usize {
    let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();
    let m = s.len();
    let n = t.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr: Vec<usize> = vec![0; n + 1];

    println!("Initial prev: {:?}", prev);

    for i in 1..=m {
        curr[0] = i;
        println!(
            "\n=== Ligne i={} (source[..i] = {:?}) ===",
            i,
            &s[..i].iter().collect::<String>()
        );

        for j in 1..=n {
            let sc = s[i - 1];
            let tc = t[j - 1];

            if sc == tc {
                curr[j] = prev[j - 1];
                println!(
                    "  (i={}, j={}) comparer {:?} et {:?} -> égaux, copie diagonale {}",
                    i,
                    j,
                    sc,
                    tc,
                    prev[j - 1]
                );
            } else {
                let del = prev[j] + 1; // suppression
                let ins = curr[j - 1] + 1; // insertion
                let sub = prev[j - 1] + 1; // substitution

                curr[j] = del.min(ins).min(sub);

                println!(
                    "  (i={}, j={}) comparer {:?} et {:?} -> diff, candidats: del={}, ins={}, sub={} => choisi={}",
                    i, j, sc, tc, del, ins, sub, curr[j]
                );
            }
        }

        println!("Fin de ligne i={}: curr = {:?}", i, curr);

        // swap
        std::mem::swap(&mut prev, &mut curr);
        println!("Après swap: prev = {:?}", prev);
    }

    println!("\nRésultat final = {}", prev[n]);
    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let source = "alignment";
        let target = "assignment";

        println!(
            "It's necessary to make {} change(s) to {:?} to get {:?}",
            edit_distance(source, target),
            source,
            target
        );
    }
}
