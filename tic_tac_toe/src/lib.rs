pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let o_wins = diagonals('O', table) || horizontal('O', table) || vertical('O', table);
    let x_wins = diagonals('X', table) || horizontal('X', table) || vertical('X', table);

    if o_wins {
        "player O won".to_string()
    } else if x_wins {
        "player X won".to_string()
    } else {
        "tie".to_string()
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let main = table[0][0] == player && table[1][1] == player && table[2][2] == player;
    let anti = table[0][2] == player && table[1][1] == player && table[2][0] == player;
    main || anti
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    let main = table[0][0] == player && table[0][1] == player && table[0][2] == player;
    let anti = table[1][0] == player && table[1][1] == player && table[1][2] == player;
    let social = table[2][0] == player && table[2][1] == player && table[2][2] == player;

    main || anti || social
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    let main = table[0][0] == player && table[1][0] == player && table[2][0] == player;
    let anti = table[0][1] == player && table[1][1] == player && table[2][1] == player;
    let social = table[0][2] == player && table[1][2] == player && table[2][2] == player;

    main || anti || social
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']])
        );
        // tie
        println!(
            "{}",
            tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']])
        );
        // player O won

        let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

        println!("{}", tic_tac_toe(diag));
        // player X won
    }
}
