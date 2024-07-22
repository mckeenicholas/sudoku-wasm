use wasm_bindgen::prelude::*;
use bit_set::BitSet;

fn create_board(board: &[u8; 81]) -> [[u8; 9]; 9] {
    let mut board_2d: [[u8; 9]; 9] = [[0; 9]; 9];
    for (i, &value) in board.iter().enumerate() {
        let row = i / 9;
        let col = i % 9;
        board_2d[row][col] = value;
    }

    board_2d
}

fn flatten_board(board: &[[u8; 9]; 9]) -> [u8; 81] {
    let mut flat_board: [u8; 81] = [0; 81];
    for row in 0..9 {
        for col in 0..9 {
            flat_board[row * 9 + col] = board[row][col];
        }
    }

    flat_board
}

fn get_cell(row: usize, col: usize) -> usize {
    (row / 3) * 3 + col / 3
}

fn next_item(board: &[[u8; 9]; 9], row: usize, col: usize) -> (usize, usize) {
    let mut pos = row * 9 + col;
    while pos < 81 && board[pos / 9][pos % 9] != 0 {
        pos += 1;
    }

    (pos / 9, pos % 9)
}

fn solve(
    board: &mut [[u8; 9]; 9],
    row: usize,
    col: usize,
    row_set: &mut [BitSet; 9],
    col_set: &mut [BitSet; 9],
    cell_set: &mut [BitSet; 9],
) -> bool {
    if row == 9 {
        return true;
    }

    let cell = get_cell(row, col);
    let available: BitSet = row_set[row]
        .union(&col_set[col])
        .collect::<BitSet>()
        .union(&cell_set[cell])
        .collect::<BitSet>();
    if available.len() == 9 {
        return false;
    }

    for i in 0..9 {
        if !available.contains(i) {
            board[row][col] = i as u8 + 1;
            row_set[row].insert(i as usize);
            col_set[col].insert(i as usize);
            cell_set[cell].insert(i as usize);

            let (new_row, new_col) = next_item(&board, row, col);

            if new_row == 9 {
                return true;
            }

            if solve(board, new_row, new_col, row_set, col_set, cell_set) {
                return true;
            }

            row_set[row].remove(i as usize);
            col_set[col].remove(i as usize);
            cell_set[cell].remove(i as usize);
        }
    }

    board[row][col] = 0;
    false
}

#[wasm_bindgen]
pub fn solve_puzzle(input_board: &[u8]) -> Vec<u8> {
    let mut fixed_board = [0u8; 81];
    fixed_board.copy_from_slice(input_board);
    let mut board = create_board(&fixed_board);

    let mut rows: [BitSet; 9] = Default::default();
    let mut columns: [BitSet; 9] = Default::default();
    let mut cells: [BitSet; 9] = Default::default();

    for i in 0..9 {
        rows[i] = BitSet::with_capacity(9);
        columns[i] = BitSet::with_capacity(9);
        cells[i] = BitSet::with_capacity(9);
    }

    for row in 0..9 {
        for col in 0..9 {
            let digit = board[row][col];
            if digit != 0 {
                let bit_set_idx = digit - 1;
                rows[row].insert(bit_set_idx as usize);
                columns[col].insert(bit_set_idx as usize);
                let cell = get_cell(row, col);
                cells[cell].insert(bit_set_idx as usize);
            }
        }
    }

    let (row, col) = next_item(&board, 0, 0);
    solve(&mut board, row, col, &mut rows, &mut columns, &mut cells);

    flatten_board(&board).to_vec()
}
