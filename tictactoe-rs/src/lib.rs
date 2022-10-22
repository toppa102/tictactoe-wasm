use std::cmp::{max, min};
use wasm_bindgen::prelude::*;

const AI: u8 = 1;
const PLAYER: u8 = 2;

/// Return i64::MAX on ai win and i64::MIN on player win
/// otherwise returns evaluation of board.
//  Evaluation is based on how many ai or player
//  has got in a row by using a variable "total" and adding
//  or subtracting from it depending on how many ai or player
//  has in a row.
#[wasm_bindgen]
pub fn check_winner(board: &[u8], size: usize, win_amount: usize) -> i64 {
    let mut total = 0;

    for y in 0..size {
        let mut amount = 0;
        let mut last = 0;
        for x in 0..size {
            let current = board[x + y * size];
            if current != last {
                last = current;
                amount = 0;
            }

            amount += 1;

            if current == AI {
                total += amount * amount;
            } else if current == PLAYER {
                total -= amount * amount;
            }

            if amount >= win_amount as i64 && current != 0 {
                if current == AI {
                    return i64::MAX;
                } else {
                    return i64::MIN;
                }
            }
        }
    }

    for x in 0..size {
        let mut amount = 0;
        let mut last = 0;
        for y in 0..size {
            let current = board[x + y * size];
            if current != last {
                last = current;
                amount = 0;
            }

            amount += 1;

            if current == AI {
                total += amount * amount;
            } else if current == PLAYER {
                total -= amount * amount;
            }

            if amount >= win_amount as i64 && current != 0 {
                if current == AI {
                    return i64::MAX;
                } else {
                    return i64::MIN;
                }
            }
        }
    }

    for dia in 0..size - win_amount + 1 {
        let mut amount = 0;
        let mut last = 0;
        for i in 0..size - dia {
            let current = board[(dia + i) + i * size];
            if current != last {
                last = current;
                amount = 0;
            }

            amount += 1;

            if current == AI {
                total += amount * amount;
            } else if current == PLAYER {
                total -= amount * amount;
            }

            if amount >= win_amount as i64 && current != 0 {
                if current == AI {
                    return i64::MAX;
                } else {
                    return i64::MIN;
                }
            }
        }
        let mut amount = 0;
        let mut last = 0;
        if dia > 0 {
            for i in 1..size - dia {
                let current = board[i + (dia + i) * size];
                if current != last {
                    last = current;
                    amount = 0;
                }

                amount += 1;

                if current == AI {
                    total += amount * amount;
                } else if current == PLAYER {
                    total -= amount * amount;
                }

                if amount >= win_amount as i64 && current != 0 {
                    if current == AI {
                        return i64::MAX;
                    } else {
                        return i64::MIN;
                    }
                }
            }
        }
    }

    for dia in 0..size - win_amount + 1 {
        let mut amount = 0;
        let mut last = 0;
        for i in 0..size - dia {
            let current = board[(size - 1 - dia - i) + i * size];

            if current != last {
                last = current;
                amount = 0;
            }

            amount += 1;

            if current == AI {
                total += amount * amount;
            } else if current == PLAYER {
                total -= amount * amount;
            }

            if amount >= win_amount as i64 && current != 0 {
                if current == AI {
                    return i64::MAX;
                } else {
                    return i64::MIN;
                }
            }
        }
        let mut amount = 0;
        let mut last = 0;
        if dia > 0 {
            for i in 0..size - dia {
                let current = board[size - 1 - i + (dia + i) * size];

                if current != last {
                    last = current;
                    amount = 0;
                }

                amount += 1;

                if current == AI {
                    total += amount * amount;
                } else if current == PLAYER {
                    total -= amount * amount;
                }

                if amount >= win_amount as i64 && current != 0 {
                    if current == AI {
                        return i64::MAX;
                    } else {
                        return i64::MIN;
                    }
                }
            }
        }
    }

    return total;
}

#[wasm_bindgen]
pub fn find_best_move(board: Vec<u8>, size: usize, win_amount: usize, max_depth: usize) -> usize {
    let mut best_value = i64::MIN;
    let mut best_move = usize::MIN;

    for i in 0..size * size {
        if board[i] == 0 {
            let mut board = board.clone();
            board[i] = AI;
            let val = minimax(
                board,
                size,
                win_amount,
                false,
                1,
                max_depth,
                i64::MIN,
                i64::MAX,
            );
            if val > best_value {
                best_value = val;
                best_move = i;
            }
        }
    }

    return best_move;
}

pub fn minimax(
    board: Vec<u8>,
    size: usize,
    win_amount: usize,
    is_maximing: bool,
    depth: usize,
    max_depth: usize,
    alpha: i64,
    beta: i64,
) -> i64 {
    let value = check_winner(&board, size, win_amount);

    if value == i64::MAX {
        return i64::MAX - depth as i64;
    }
    if value == i64::MIN {
        return i64::MIN + depth as i64;
    }

    if depth >= max_depth {
        return value - depth as i64;
    }

    if board.iter().all(|x| *x != 0) {
        return 0;
    }

    if is_maximing {
        let mut best = i64::MIN;
        let mut alpha = alpha;
        for i in 0..size * size {
            if board[i] == 0 {
                let mut board = board.clone();
                board[i] = AI;
                let value = minimax(
                    board,
                    size,
                    win_amount,
                    false,
                    depth + 1,
                    max_depth,
                    alpha,
                    beta,
                );
                best = max(best, value);
                alpha = max(alpha, best);
                if beta <= alpha {
                    break;
                }
            }
        }

        return best;
    } else {
        let mut best = i64::MAX;
        let mut beta = beta;
        for i in 0..size * size {
            if board[i] == 0 {
                let mut board = board.clone();
                board[i] = PLAYER;
                let value = minimax(
                    board,
                    size,
                    win_amount,
                    true,
                    depth + 1,
                    max_depth,
                    alpha,
                    beta,
                );
                best = min(best, value);
                beta = min(beta, best);
                if beta <= alpha {
                    break;
                }
            }
        }

        return best;
    }
}
