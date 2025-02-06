// use std::any::Any;
// use std::collections::HashSet;
// use std::io::empty;
// use std::env;


#[derive(Copy, Clone)]
struct Tile{
    found: bool,
    final_guess: i8, // for testing, but the found solution
    possible: [bool; 9], // all possible solutions, true if available, false if not 0 = 1, so 8 = 9
}

impl Tile {
    fn new_empty() -> Self {
        Tile {
            found: false,
            final_guess: -1,
            possible: [true; 9],
        }
    }
    fn new_found(fg: i8) -> Self{
        Tile{
            found: true,
            final_guess: fg,
            possible: [false; 9],
        }

    }
}

struct Board{
    _board: [[Tile; 9];9],
}

impl Board {
    fn new_empty() -> Self{
        Board{
            _board: [[Tile::new_empty();9];9],
        }
    }

    fn new(input: [[Tile; 9]; 9]) -> Self {

        Board{
            _board: input,
        }
    }

}


fn main() {

    let board = ReadInNewBoard("Sudoku1.txt");

    let mut complete: bool = true;


    while complete{







    }



}
use std::fs::read_to_string;
fn ReadInNewBoard(filename: &str) -> Board {

    //let mut result = Vec::new();
    let mut output: Board = Board::new_empty();

    let mut increment: i32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        for ch in line.chars() {
            let t = if ch == '_' {
                Tile::new_empty()
            } else {
                let p = ch.to_string().parse::<i8>().unwrap();
                Tile::new_found(p)
            };

            let x = increment % 9;
            let y = (increment / 9) % 9;

            //println!("{x}, {y}, {}", t.final_guess); //for testing purposes

            // Fixed the array indexing - note we're using y first since it's a 2D array
            output._board[y as usize][x as usize] = t;

            increment += 1;
        }
    }



    return output;
}



/*fn EvaluateBoard(board: Board) -> Board {


    return null;
}*/

fn evaluate_tile(tile_vec: (i8, i8), board: Board) -> [bool; 9] {

    let column_eval = evaluate_by_column(tile_vec, &board);
    let row_eval = evaluate_by_row(tile_vec, &board);
    let square_eval = evaluate_square(tile_vec, &board);


    let mut result = [true; 9];
    for i in 0..8{
        result[i] = column_eval[i] && row_eval[i] && square_eval[i];
    }

    result
}


// returns an array of booleans, the index is the possible number(-1) and true means possible valid, false means invalid
fn evaluate_by_column(tile_vec: (i8, i8), board: &Board) -> [bool; 9]{

    let column = board._board[tile_vec.0]; // 0=x 1=y

    let mut checking_array = [true; 9];


    // ideally this will always work, although there isnt any guess work to find incorect options
    for element in column.iter(){
        if(element.found){
            checking_array[element.final_guess] = false;
        }
    }


    checking_array
}

fn evaluate_by_row(tile_vec: (i8, i8), board: &Board) -> [bool; 9]{

    let mut row: [Tile; 9] = [Tile::new_empty(); 9];

    for i in 0..8{
        row[i] = board[i][tile_vec.1 as usize];
    }

    let mut checking_array = [true; 9];

    for element in row.iter(){
        if(element.found){
            checking_array[element.final_guess] = false;
        }
    }

    checking_array
}

fn evaluate_square(tile_vec: (i8, i8), board: &Board) -> [bool;9]{ // returns if a valid square

    let _board = board._board;

    let offsetX = tile_vec.0 % 3; // using the modulous tells us the offset to the origion(top left)
    let offsetY = tile_vec.1 % 3;

    let origion_pos_x = tile_vec.0 - offsetX;
    let origion_pos_y = tile_vec.1 - offsetY;

    let mut square: [Tile; 9] = [

        _board[origion_pos_x][origion_pos_y],
        _board[origion_pos_x + 1][origion_pos_y],
        _board[origion_pos_x + 2][origion_pos_y],
        _board[origion_pos_x][origion_pos_y + 1],
        _board[origion_pos_x][origion_pos_y + 2],
        _board[origion_pos_x + 1][origion_pos_y + 1],
        _board[origion_pos_x + 1][origion_pos_y + 2],
        _board[origion_pos_x + 2][origion_pos_y + 1],
        _board[origion_pos_x + 2][origion_pos_y + 2],

        //manually adds each tile into the array

    ];

    let mut checking_array : [bool; 9] = [true; 9];

    for element in square.iter(){
        if(element.found){
            checking_array[element.final_guess] = false;
        }
    }


    checking_array
}