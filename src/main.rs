use std::io;
use std::io::Write;

#[derive(Copy, Clone, PartialEq)]
enum Pieces {
    EMPTY,
    NOUGHT,
    CROSS
}

const WINNING_PATTERNS: &'static [i32] = &[
    0b111000000,
    0b000111000,
    0b000000111,
    0b100100100,
    0b010010010,
    0b001001001,
    0b100010001,
    0b001010100
];

fn print_board (board: [Pieces; 9]) {
    for (i, piece) in board.iter().enumerate() {
        print!("| ");
        match piece {
            Pieces::EMPTY => print!(" "),
            Pieces::NOUGHT => print!("O"),
            Pieces::CROSS => print!("X"),
        }
        print!(" |");

        if (i + 1) % 3 == 0 {
            print!("\n");
        }
    }
}

fn play_move (board: &mut [Pieces; 9], space: i32, piece: Pieces) -> bool {
    if piece != Pieces::EMPTY && board[space as usize] != Pieces::EMPTY {
        return false;
    }

    board[space as usize] = piece;
    return true;
}

fn has_won (board: [Pieces; 9], piece: Pieces) -> bool {
    let mut board_state = 0;

    for (i, val) in board.iter().enumerate() {
        let s = if *val == piece { 1 } else { 0 };

        board_state = board_state | (s << (8 - i));
    }

    for winning_pattern in WINNING_PATTERNS {
        if *winning_pattern & board_state == *winning_pattern {
            return true;
        }
    }

    return false;
}

fn is_full (board: [Pieces; 9]) -> bool {
    for val in board {
        if val == Pieces::EMPTY {
            return false;
        }
    }

    return true;
}

fn take_input (input: &mut String) -> i32 { 
    let _ = io::stdin().read_line(input);
    let output = match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(n) => panic!("There was an error trying to convert {n}")
    };


    if output < 0 || output > 8 {
        println!("Please enter a valid integer between 0 & 8!");
        return -1;
    }

    return output;
}

fn available_moves (board: [Pieces; 9]) -> Vec<i32> {
    let mut moves = Vec::<i32>::new();

    for (i, val) in board.iter().enumerate() {
        if *val != Pieces::EMPTY {
            continue;
        }

        moves.push(i.try_into().unwrap());
    }

    return moves;
}

fn minimax (mut board: [Pieces; 9], player: bool) -> (i32, i32) {
    let ai_won: bool = has_won(board, Pieces::CROSS);
    let player_won: bool = has_won(board, Pieces::NOUGHT);
     
    if is_full(board) || ai_won || player_won {
        if ai_won {
            return (-1, -1);
        } else if player_won {
            return (1, -1);
        } else {
            return (0, -1);
        }
    } else {
        let mut best_idx: i32 = -1;
        let mut best: i32 = if player { std::i32::MIN } else { std::i32::MAX };

        for m in available_moves(board) {
            if player {
                play_move(&mut board, m, Pieces::NOUGHT);
                let (b, _) = minimax(board, false); 
                play_move(&mut board, m, Pieces::EMPTY);

                if b > best {
                    best = b;
                    best_idx = m;
                }
            } else {
                play_move(&mut board, m, Pieces::CROSS);
                let (b, _) = minimax(board, true); 
                play_move(&mut board, m, Pieces::EMPTY);

                if b < best {
                    best = b;
                    best_idx = m;
                }
            }
        }

        return (best, best_idx);
    }
} 

fn main() {
    let mut board: [Pieces; 9] = [Pieces::EMPTY; 9];
    let mut input: String = String::new();
    let mut turn: i8 = 0;

    print_board(board);

    loop {
        if turn % 2 == 0 {
            print!("Please enter your move: ");
            std::io::stdout().flush().unwrap();
            let index: i32 = take_input(&mut input);
            input.clear();
        
            if index == -1 {
                continue; 
            }

            if !play_move(&mut board, index, Pieces::NOUGHT) {
                continue;
            }
        } else {
            let (_, best_idx) = minimax(board, false);

            play_move(&mut board, best_idx, Pieces::CROSS); 
            print_board(board);
        }

        turn += 1;

        let winner = has_won(board, Pieces::NOUGHT) || has_won(board, Pieces::CROSS);

        if winner {
            break;
        }
    }
}
