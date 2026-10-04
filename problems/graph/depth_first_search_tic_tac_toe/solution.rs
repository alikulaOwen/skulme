/*
Tic-Tac-Toe Depth First Search Rust Demo
Copyright 2021 David V. Makray

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#[allow(unused_imports)]
use std::io;

//Interactive Tic-Tac-Toe play needs the "rand = "0.8.3" crate.
//#[cfg(not(test))]
//extern crate rand;
//#[cfg(not(test))]
//use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Players {
    Blank,
    PlayerX,
    PlayerO,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct SinglePlayAction {
    position: Position,
    side: Players,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayActions {
    positions: Vec<Position>,
    side: Players,
}

#[allow(dead_code)]
#[cfg(not(test))]
fn main() {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (main)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement main");
}

#[allow(dead_code)]
fn display_board(board: &[Vec<Players>]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (display_board)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement display_board");
}

fn available_positions(board: &[Vec<Players>]) -> Vec<Position> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (available_positions)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement available_positions");
}

fn win_check(player: Players, board: &[Vec<Players>]) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (win_check)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement win_check");
}

//Minimize the actions of the opponent while maximizing the game state of the current player.
pub fn minimax(side: Players, board: &[Vec<Players>]) -> Option<PlayActions> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (minimax)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement minimax");
}

//Promote only better or collate equally scored game plays
fn append_playaction(
    current_side: Players,
    opt_play_actions: &mut Option<PlayActions>,
    appendee: SinglePlayAction,
) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (append_playaction)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement append_playaction");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn win_state_check() {
        let mut board = vec![vec![Players::Blank; 3]; 3];
        board[0][0] = Players::PlayerX;
        board[0][1] = Players::PlayerX;
        board[0][2] = Players::PlayerX;
        let responses = minimax(Players::PlayerO, &board);
        assert_eq!(responses, None);
    }

    #[test]
    fn win_state_check2() {
        let mut board = vec![vec![Players::Blank; 3]; 3];
        board[0][0] = Players::PlayerX;
        board[0][1] = Players::PlayerO;
        board[1][0] = Players::PlayerX;
        board[1][1] = Players::PlayerO;
        board[2][1] = Players::PlayerO;
        let responses = minimax(Players::PlayerO, &board);
        assert_eq!(responses, None);
    }

    #[test]
    fn block_win_move() {
        let mut board = vec![vec![Players::Blank; 3]; 3];
        board[0][0] = Players::PlayerX;
        board[0][1] = Players::PlayerX;
        board[1][2] = Players::PlayerO;
        board[2][2] = Players::PlayerO;
        let responses = minimax(Players::PlayerX, &board);
        assert_eq!(
            responses,
            Some(PlayActions {
                positions: vec![Position { x: 2, y: 0 }],
                side: Players::PlayerX
            })
        );
    }

    #[test]
    fn block_move() {
        let mut board = vec![vec![Players::Blank; 3]; 3];
        board[0][1] = Players::PlayerX;
        board[0][2] = Players::PlayerO;
        board[2][0] = Players::PlayerO;
        let responses = minimax(Players::PlayerX, &board);
        assert_eq!(
            responses,
            Some(PlayActions {
                positions: vec![Position { x: 1, y: 1 }],
                side: Players::Blank
            })
        );
    }

    #[test]
    fn expected_loss() {
        let mut board = vec![vec![Players::Blank; 3]; 3];
        board[0][0] = Players::PlayerX;
        board[0][2] = Players::PlayerO;
        board[1][0] = Players::PlayerX;
        board[2][0] = Players::PlayerO;
        board[2][2] = Players::PlayerO;
        let responses = minimax(Players::PlayerX, &board);
        assert_eq!(
            responses,
            Some(PlayActions {
                positions: vec![
                    Position { x: 1, y: 0 },
                    Position { x: 1, y: 1 },
                    Position { x: 2, y: 1 },
                    Position { x: 1, y: 2 }
                ],
                side: Players::PlayerO
            })
        );
    }
}
