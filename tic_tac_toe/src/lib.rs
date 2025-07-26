pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        return "player X won".to_string();
    } else if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        return "player O won".to_string();
    } else {
        return "tie".to_string();
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][1] == player && table[2][2] == player) ||
        (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][0] == player && table[2][0] == player) ||
        (table[0][1] == player && table[1][1] == player && table[2][1] == player) ||
        (table[0][2] == player && table[1][2] == player && table[2][2] == player)
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[0][1] == player && table[0][2] == player) ||
        (table[1][0] == player && table[1][1] == player && table[1][2] == player) ||
        (table[2][0] == player && table[2][1] == player && table[2][2] == player)
}
