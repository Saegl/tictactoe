use core::panic;
use std::io::stdin;

#[derive(Clone)]
struct TicTacToeState {
    cells: [i8; 9],     // 0 = empty, 1 = X, 2 = O
    side_to_play: bool, // true if X to play
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Xwon,
    Owon,
    Draw,
    Playing,
}

use crate::Outcome::*;

impl Outcome {
    fn is_end(&self) -> bool {
        *self == Xwon || *self == Owon || *self == Draw
    }
}

fn view_cell(val: i8) -> String {
    if val == 1 {
        "X".to_string()
    } else if val == 2 {
        "O".to_string()
    } else if val == 0 {
        " ".to_string()
    } else {
        panic!("Wrong value of cell")
    }
}

impl TicTacToeState {
    fn view(&self) -> String {
        let label: String = match self.outcome() {
            Xwon => "X is won".to_string(),
            Owon => "O is won".to_string(),
            Draw => "Game ended with a draw".to_string(),
            Playing => format!("{} to play", if self.side_to_play { "X" } else { "O" }),
        };
        format!(
            "-------\n|{} {} {}|\n|{} {} {}|\n|{} {} {}|\n-------\n",
            view_cell(self.cells[0]),
            view_cell(self.cells[1]),
            view_cell(self.cells[2]),
            view_cell(self.cells[3]),
            view_cell(self.cells[4]),
            view_cell(self.cells[5]),
            view_cell(self.cells[6]),
            view_cell(self.cells[7]),
            view_cell(self.cells[8]),
        ) + &label
    }
}

impl Default for TicTacToeState {
    fn default() -> Self {
        TicTacToeState {
            cells: [0; 9],
            side_to_play: true,
        }
    }
}

impl TicTacToeState {
    fn actions(&self) -> Vec<usize> {
        let mut ans = vec![];
        for (i, cell) in self.cells.iter().enumerate() {
            if *cell == 0 {
                ans.push(i)
            }
        }
        ans
    }
    fn is_player_win(&self, p: i8) -> bool {
        (
            // Rows
            self.cells[0] == p) && (self.cells[1] == p) && (self.cells[2] == p)
            || (self.cells[3] == p) && (self.cells[4] == p) && (self.cells[5] == p)
            || (self.cells[6] == p) && (self.cells[7] == p) && (self.cells[8] == p)
            // Colums
            || (self.cells[0] == p) && (self.cells[3] == p) && (self.cells[6] == p)
            || (self.cells[1] == p) && (self.cells[4] == p) && (self.cells[7] == p)
            || (self.cells[2] == p) && (self.cells[5] == p) && (self.cells[8] == p)
            // Diagonals
            || (self.cells[0] == p) && (self.cells[4] == p) && (self.cells[8] == p)
            || (self.cells[6] == p) && (self.cells[4] == p) && (self.cells[2] == p)
    }
    fn is_x_win(&self) -> bool {
        self.is_player_win(1)
    }
    fn is_o_win(&self) -> bool {
        self.is_player_win(2)
    }
    fn no_empty_cells(&self) -> bool {
        let mut ans = true;
        for cell in self.cells {
            if cell == 0 {
                ans = false;
                break;
            }
        }
        ans
    }
    fn outcome(&self) -> Outcome {
        if self.is_x_win() {
            Outcome::Xwon
        } else if self.is_o_win() {
            Outcome::Owon
        } else if self.no_empty_cells() {
            Outcome::Draw
        } else {
            Outcome::Playing
        }
    }
    fn is_legal(&self, action: usize) -> bool {
        self.cells[action] == 0
    }
    fn play(&self, action: usize) -> TicTacToeState {
        if !self.is_legal(action) {
            panic!("Illegal action");
        }
        let mut new_state = self.clone();
        new_state.cells[action] = if self.side_to_play { 1 } else { 2 };
        new_state.side_to_play = !new_state.side_to_play;
        new_state
    }
}

fn evaluate(state: &TicTacToeState) -> f32 {
    match state.outcome() {
        Xwon => 1.0,
        Owon => -1.0,
        Draw => 0.0,
        Playing => panic!("You should call evaluate only on terminal states"),
    }
}

// returns (action, value)
fn minimax(state: TicTacToeState) -> (usize, f32) {
    if state.outcome().is_end() {
        return (0, evaluate(&state));
    }
    let best_value = 0.0;
    let best_action = 0;

    for action in state.actions() {
        let new_state = state.play(action);
        minimax(new_state);
    }

    (0, 0.0)
}

fn main() {
    let mut state: TicTacToeState = Default::default();

    while !state.outcome().is_end() {
        println!("{}", state.view());
        println!("{:?}", state.outcome());
        print!("Available actions: ");
        for action in state.actions() {
            print!("{} ", action);
        }
        println!();

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Cannot read string");

        if input.trim() == "quit" {
            break;
        }
        let user_action: usize = input.trim().parse().expect("You should input number");

        println!("You action is {}", user_action);
        state = state.play(user_action);
    }

    println!("{}", state.view());
}
