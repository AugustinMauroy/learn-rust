const BOARD_SIZE: usize = 9;

struct Game {
    board: [char; BOARD_SIZE],
    player: char,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [' '; BOARD_SIZE],
            player: 'X',
        }
    }

    fn display(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!(
            " {} | {} | {} ",
            self.board[0], self.board[1], self.board[2]
        );
        println!("---|---|---");
        println!(
            " {} | {} | {} ",
            self.board[3], self.board[4], self.board[5]
        );
        println!("---|---|---");
        println!(
            " {} | {} | {} ",
            self.board[6], self.board[7], self.board[8]
        );
    }

    fn check_winner(&self) -> Option<char> {
        let lines = [
            // rows
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // columns
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // diagonals
            [0, 4, 8],
            [2, 4, 6],
        ];
        for line in lines.iter() {
            if self.board[line[0]] != ' '
                && self.board[line[0]] == self.board[line[1]]
                && self.board[line[1]] == self.board[line[2]]
            {
                return Some(self.board[line[0]]);
            }
        }
        None
    }
}

fn main() {
    println!("Welcome to Tic Tac Toe!");

    println!("Player 1 is X and Player 2 is O.");
    println!("Player 1 goes first.");
    println!("To make a move, enter a number from 0 to 8.");
    println!("Press any key to start.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    drop(input);
    let mut game = Game::new();

    game.display();

    loop {
        println!("Player {}, please make a move.", game.player);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let index: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number from 0 to 8.");
                continue;
            }
        };
        if index >= BOARD_SIZE || game.board[index] != ' ' {
            println!("Invalid move!");
            continue;
        }
        game.board[index] = game.player;
        game.display();

        if let Some(winner) = game.check_winner() {
            println!("Player {} wins!", winner);
            break;
        }

        if !game.board.contains(&' ') {
            println!("The game is a draw!");
            break;
        }

        game.player = if game.player == 'X' { 'O' } else { 'X' };
    }
}
