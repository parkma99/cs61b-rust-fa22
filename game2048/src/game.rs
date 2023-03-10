use std::fmt::Debug;

use crate::{bindings::log, random::Random};

const GENERATE_2_PROBABILITY: f64 = 0.9; // otherwise generates a 4
const MAX_PIECE: u32 = 2048;

pub struct Board {
    pub tiles: Vec<Vec<u32>>, // indexed by [x][y], (0,0) at top left
    pub width: usize,
    pub height: usize,
}

struct RotatedBoard {
    tiles: Vec<Vec<u32>>, // indexed by [x][y], (0,0) at top left
    width: usize,
    height: usize,
    dir: Direction, // direction that is facing up
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MovingTile {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
    pub value: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Board {
    pub fn new(width: usize, height: usize, random: &mut Random) -> Self {
        let mut board = Self {
            tiles: vec![vec![0; height]; width],
            width,
            height,
        };
        add_tile(&mut board, random);
        add_tile(&mut board, random);
        board
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile_strs = (0..self.height)
            .map(|h| {
                (0..self.width)
                    .map(|w| self.tiles[w][h].to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();

        writeln!(
            f,
            "Board(width:{}, height:{}){:#?}",
            self.width, self.height, tile_strs
        )
    }
}

impl MovingTile {
    pub fn new(start_x: usize, start_y: usize, end_x: usize, end_y: usize, value: u32) -> Self {
        MovingTile {
            start_x,
            start_y,
            end_x,
            end_y,
            value,
        }
    }
}

// given a location (x,y) on the rotated board,
// returns the original location of the tile on the unrotated board
fn unrotate(rotated: &RotatedBoard, x: usize, y: usize) -> (usize, usize) {
    match rotated.dir {
        Direction::North => (x, y),
        Direction::East => (rotated.height - 1 - y, x),
        Direction::South => (rotated.width - 1 - x, rotated.height - 1 - y),
        Direction::West => (y, rotated.width - 1 - x),
    }
}

// Rotates the board such that the given direction faces up.
// Tiles should "fall" upwards after this rotation
fn rotate_board(board: &Board, dir: Direction) -> RotatedBoard {
    let (new_width, new_height) = match dir {
        Direction::North | Direction::South => (board.width, board.height),
        Direction::East | Direction::West => (board.height, board.width),
    };
    let mut rotated = RotatedBoard {
        tiles: vec![vec![0; new_height]; new_width],
        width: new_width,
        height: new_height,
        dir,
    };
    for x in 0..new_width {
        for y in 0..new_height {
            let (old_a, old_b) = unrotate(&rotated, x, y);
            rotated.tiles[x][y] = board.tiles[old_a][old_b];
        }
    }
    rotated
}

// Rotate the board back to its original position (north facing up)
fn unrotate_board(rotated: RotatedBoard) -> Board {
    let (old_width, old_height) = match rotated.dir {
        Direction::North | Direction::South => (rotated.width, rotated.height),
        Direction::East | Direction::West => (rotated.height, rotated.width),
    };
    let mut board = Board {
        tiles: vec![vec![0; old_height]; old_width],
        width: old_width,
        height: old_height,
    };
    for a in 0..rotated.width {
        for b in 0..rotated.height {
            let (old_a, old_b) = unrotate(&rotated, a, b);
            board.tiles[old_a][old_b] = rotated.tiles[a][b];
        }
    }
    board
}

// Given moves constructed on a rotated board, rotate them to match
// the board's original position
fn unrotate_move(rotated: &RotatedBoard, mt: &mut MovingTile) {
    (mt.start_x, mt.start_y) = unrotate(rotated, mt.start_x, mt.start_y);
    (mt.end_x, mt.end_y) = unrotate(rotated, mt.end_x, mt.end_y);
}

// Given the current state of the board and the direction in which to tilt,
// return the new state of the board,
// a list of where all the current tiles have moved (even the ones that stay stationary),
// and how much to increment the score by
// If the board doesn't change, return None
pub fn tilt(board: &Board, dir: Direction) -> Option<(Board, Vec<MovingTile>, u32)> {
    log!("This is an example log message. Use log messages to help you debug!");

    let mut rotated = rotate_board(board, dir);
    let mut score = 0;
    let mut movings: Vec<MovingTile> = Vec::new();
    let mut is_changed = false;

    for col in 0..rotated.width {
        let mut row = 0;
        let mut is_merged = false;
        let mut is_moving = false;
        let mut last_merged = 0;
        let mut is_checked = false;
        while row < rotated.height {
            let cur = rotated.tiles[col][row];
            if cur != 0 && !is_moving && !is_checked {
                movings.push(MovingTile::new(col, row, col, row, cur));
            }
            match next_not_null_tile_row(&rotated, col, row) {
                None => {
                    break;
                }
                Some(pos) => {
                    if last_merged != row {
                        is_merged = false;
                    }
                    let next = rotated.tiles[col][pos];
                    if cur == 0 || !is_merged && cur == next {
                        if cur != 0 && row != pos {
                            score += next * 2;
                            is_merged = true;
                            last_merged = row;
                        }
                        is_changed = true;
                        is_moving = true;
                        rotated.tiles[col][pos] = 0;
                        rotated.tiles[col][row] += next;
                        movings.push(MovingTile::new(col, pos, col, row, next));
                    }
                    if row == pos {
                        break;
                    }
                    if is_checked || is_merged {
                        is_checked = false;
                        row += 1;
                    } else {
                        is_checked = true;
                    }
                }
            }
        }
    }

    match is_changed {
        false => None,
        true => {
            for moving in movings.iter_mut() {
                unrotate_move(&rotated, moving)
            }
            let result = unrotate_board(rotated);
            Some((result, movings, score))
        }
    }
}
fn next_not_null_tile_row(rotated: &RotatedBoard, col: usize, row: usize) -> Option<usize> {
    (row + 1..rotated.height).find(|&pos| rotated.tiles[col][pos] != 0)
}

// Add a random tile to the given board
// does nothing if there are no spaces on the board
pub fn add_tile(board: &mut Board, random: &mut Random) {
    let mut open_positions = 0;
    for x in 0..board.width {
        for y in 0..board.height {
            if board.tiles[x][y] == 0 {
                open_positions += 1;
            }
        }
    }

    if open_positions == 0 {
        return;
    }
    let new_value = if random.next_f64() > GENERATE_2_PROBABILITY {
        4
    } else {
        2
    };
    let mut idx = random.next_below(open_positions);
    for x in 0..board.width {
        for y in 0..board.height {
            if board.tiles[x][y] == 0 {
                if idx == 0 {
                    board.tiles[x][y] = new_value;
                    return;
                } else {
                    idx -= 1;
                }
            }
        }
    }
}

// returns whether the game is over
// the game is over if there are no possible moves left
pub fn game_over(board: &Board) -> bool {
    if max_tile_exists(board) || at_least_one_move_exists(board) {
        return false;
    }
    true
}

fn max_tile_exists(board: &Board) -> bool {
    traversing_utils(board, |board: &Board, x: usize, y: usize| {
        board.tiles[x][y] == MAX_PIECE
    })
}
fn at_least_one_move_exists(board: &Board) -> bool {
    if empty_space_exists(board) {
        return true;
    }
    traversing_utils(board, |board: &Board, x: usize, y: usize| {
        if x > 0 && x < board.width - 1 && y > 0 && y < board.height - 1 {
            let cur = board.tiles[x][y];
            let left = board.tiles[x - 1][y];
            let right = board.tiles[x + 1][y];
            let up = board.tiles[x][y - 1];
            let down = board.tiles[x][y + 1];
            return left == cur || right == cur || up == cur || down == cur;
        }
        if (x == 0 || x == board.width - 1) && y > 0 && y < board.height - 1 {
            let cur = board.tiles[x][y];
            let up = board.tiles[x][y - 1];
            let down = board.tiles[x][y + 1];
            return up == cur || down == cur;
        }
        if (y == 0 || y == board.height - 1) && x > 0 && x < board.width - 1 {
            let cur = board.tiles[x][y];
            let left = board.tiles[x - 1][y];
            let right = board.tiles[x + 1][y];
            return left == cur || right == cur;
        }
        false
    })
}
fn empty_space_exists(board: &Board) -> bool {
    traversing_utils(board, |board: &Board, x: usize, y: usize| {
        board.tiles[x][y] == 0
    })
}

fn traversing_utils(board: &Board, func: fn(&Board, usize, usize) -> bool) -> bool {
    for a in 0..board.width {
        for b in 0..board.height {
            if func(board, a, b) {
                return true;
            }
        }
    }
    false
}
