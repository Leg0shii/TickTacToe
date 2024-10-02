use std::io;

fn main() {
    send_welcome_message();

    let mut player_1: String = String::new();
    let mut player_2: String = String::new();

    set_player_names(&mut player_1, &mut player_2);
    send_start_message(&player_1, &player_2);

    let mut rows = init_game();
    let mut winner = 0;
    let mut curr_player = 1;
    let mut empty_spaces = 9;
    
    while winner == 0 {
        if curr_player == 1 {
            do_game_tick(&mut rows, &player_1, curr_player);
            curr_player = 2;
        } else {
            do_game_tick(&mut rows, &player_2, curr_player);
            curr_player = 1;
        }
        winner = get_winner(&mut rows);
        empty_spaces = empty_spaces - 1;
        if empty_spaces == 0 {
            send_tie_message();
            return;
        }
    }

    if winner == 1 {
        send_winner_message(&player_1);
    } else if winner == 2 {
        send_winner_message(&player_2);
    }
}

fn send_welcome_message() {
    clear_window();
    println!("Herzlich Willkommen zu TicTacToe - The Dragon Experience™");
}

fn set_player_names(player_1: &mut String, player_2: &mut String) {
    println!("Spieler 1. Bitte gib nun deinen Namen ein!");
    io::stdin().read_line(player_1).expect("ERROR WHILE SETTING PLAYER NAME");
    strip_trailing_nl(player_1);
    clear_window();
    println!("Willkommen und viel Glück {}", player_1);

    println!("Spieler 2. Bitte gib nun deinen Namen ein!");
    io::stdin().read_line(player_2).expect("ERROR WHILE SETTING PLAYER NAME");
    strip_trailing_nl(player_2);
    clear_window();
    println!("Dir wünsche ich natürlich auch viel Glück, {}", player_2);
}

fn strip_trailing_nl(input: &mut String) {
    let new_len = input
        .char_indices()
        .rev()
        .find(|(_, c)| !matches!(c, '\n' | '\r'))
        .map_or(0, |(i, _)| i + 1);
    if new_len != input.len() {
        input.truncate(new_len);
    }
}

fn send_start_message(mut player_1: &String, mut player_2: &String) {
    clear_window();
    println!("Das Spielfeld wurde initialisiert und die Spieler erfolgreich gespeichert!");
    println!("Spieler 1: {}", &mut player_1);
    println!("Spieler 2: {}", &mut player_2);
    println!("Möge der Bessere gewinnen!");
}

fn send_winner_message(mut player: &String) {
    println!("Glückwunsch {}, du hast gewonnen!", player);
}

fn send_tie_message() {
    println!("Unentschieden!");
}

fn init_game() -> [[i32; 3]; 3] {
    let mut rows = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    return rows;
}

fn do_game_tick(rows: &mut [[i32; 3]; 3], mut player: &String, symbol: i32) {
    clear_window();
    draw_board(rows);
    player_move(rows, &player, symbol);
}

fn player_move(rows: &mut [[i32; 3]; 3], mut player: &String, symbol: i32) {
    let mut first: bool = true;
    let mut x = 3;
    let mut y = 3;

    while x < 0 || x > 2 || y < 0 || y > 2 || rows[x][y] != 0 {
        if first {
            println!("Du bist am Zug {} ({}). Gib nun deinen Zug im Format X,Y an!", player, symbol);
        } else {
            println!("Falsche Eingabe, bitte gib deinen Zug in diesem Format an: X,Y");
        }
        first = false;

        let mut player_input: String = String::new();
        io::stdin().read_line(&mut player_input).expect("ERROR ON PLAYER MOVE");
        x = player_input.chars().nth(0).expect("ERROR WHEN ENTERING NUMBER").to_digit(10).unwrap() as usize;
        y = player_input.chars().nth(2).expect("ERROR WHEN ENTERING NUMBER").to_digit(10).unwrap() as usize;
    }

    rows[x][y] = symbol;
}

fn draw_board(rows: &mut [[i32; 3]; 3]) {
    println!("| {} | {} | {} |", rows[0][0], rows[1][0], rows[2][0]);
    println!("| {} | {} | {} |", rows[0][1], rows[1][1], rows[2][1]);
    println!("| {} | {} | {} |", rows[0][2], rows[1][2], rows[2][2]);
}

fn get_winner(rows: &mut [[i32; 3]; 3]) -> i32 {
    if rows[0][0] == rows[0][1] && rows[0][1] == rows[0][2] {
        return rows[0][0];
    }
    if rows[1][0] == rows[1][1] && rows[1][1] == rows[1][2] {
        return rows[1][0];
    }
    if rows[2][0] == rows[2][1] && rows[2][1] == rows[2][2] {
        return rows[2][0];
    }
    if rows[0][0] == rows[1][0] && rows[1][0] == rows[2][0] {
        return rows[0][0];
    }
    if rows[0][1] == rows[1][1] && rows[1][1] == rows[2][1] {
        return rows[0][1];
    }
    if rows[0][2] == rows[1][2] && rows[1][2] == rows[2][2] {
        return rows[0][2];
    }
    if rows[0][0] == rows[1][1] && rows[1][1] == rows[2][2] {
        return rows[0][0];
    }
    if rows[2][0] == rows[1][1] && rows[1][1] == rows[0][2] {
        return rows[2][0];
    }
    return 0;
}

fn clear_window() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
}
