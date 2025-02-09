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
    fn new_found(fg: i8) -> Self{ // locks in your final guess and sets the new values
        Tile{
            found: true,
            final_guess: fg,
            possible: [false; 9],
        }

    }
    fn new_eval(old: &Tile, eval: [bool; 9]) -> Self{ // update the evaluation function
        Tile{

            found: old.found,
            final_guess: old.final_guess,
            possible: eval

        }
    }

    fn lock_in_guess(t: &Tile) -> i8{ // finds the final guess

        for i in 0..8{
            if(t.possible[i]){
                return i as i8;
            }
        }


        panic!("Paniced: could not lock in the final guess");

        -1


    }

    fn ready_to_guess(t: &Tile) -> bool{ // is the checking method to find if you can make a guess

        let mut iterator: i8 = 0;

        for i in t.possible.iter(){
            if(*i){
                iterator += 1;
            }
        }

        if(iterator == 1){
            return true;
        }
        else if iterator < 1{
            panic!("THERE IS NO POSSIBLE VALUE FOR READY");
        }



        false
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

    fn output(input: &Board) -> String{

        let mut tmp: String = String::from("");

        for x in 0..8{
            for y in 0..8{

                let string = tmp.add(input._board[x][y].final_guess.to_string().as_str());

            }
        }

        tmp

    }

    fn new(input: &Board) -> Self {

        Board{
            _board: input._board,
        }
    }

    fn finished(input: &Board) -> bool{ // tells you if the board is completed

        for x in 0..8{
            for y in 0..8{

                if !input._board[x][y].found{
                    return false;
                }
            }
        }
        true
    }


}


fn main() {

    let mut board = read_in_board("Sudoku1.txt");

    let fin = sudoku(board);

}

fn sudoku(board: Board) -> Board{

    let evaltation: Board= evaluate_whole_board(&board);

    let collapsed: Board = collapse(evaltation);

    if(Board::finished(&collapsed)){
        return collapsed;
    }
    sudoku(collapsed)
}





use std::fs::read_to_string;
fn read_in_board(filename: &str) -> Board {

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


use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Add;
use std::ptr::null;

fn output_board(filename: String, board: &Board){

    let f = File::create(filename).expect("unable to create file");
    let mut f = BufWriter::new(f);







}


fn evaluate_whole_board(input: &Board) -> Board{

    let mut _output: Board = Board::new(input);

    for x in 0..8{
        for y in 0..8{

            if(input._board[x as usize][y as usize].found){
                _output._board[x][y] = input._board[x as usize][y as usize];
            }

            let evaluation = evaluate_tile((x as i8, y as i8), input);

            _output._board[x][y] = Tile::new_eval(&input._board[x][y], evaluation);


        }
    }



    _output
}


// this functions updates the tiles "found" and "guess" atributes.
fn collapse(input: Board) -> Board{

    let mut _output: Board = Board::new(&input);

    for x in 0..8{
        for y in 0..8{

            if input._board[x as usize][y as usize].found {
                _output._board[x as usize][y as usize] = input._board[x as usize][y as usize];
            }
            else if Tile::ready_to_guess(&input._board[x as usize][y as usize]) { // if there is only 1 possible solution, then why not say that it is true!

                let guess: i8 = Tile::lock_in_guess(&input._board[x as usize][y as usize]);

                _output._board[x as usize][y as usize] = Tile::new_found(guess);

            }
            else{
                _output._board[x as usize][y as usize] = input._board[x as usize][y as usize];
            }
        }
    }


    _output
}


fn evaluate_hash(input: [bool; 9]) -> i8{

    let mut iterator: i8 = 0;

    for x in input.iter(){

        if(*x){
            iterator += 1;
        }

    }


    iterator
}



fn evaluate_tile(tile_vec: (i8, i8), board: &Board) -> [bool; 9] {

    let column_eval = evaluate_by_column(tile_vec, board);
    let row_eval = evaluate_by_row(tile_vec, board);
    let square_eval = evaluate_square(tile_vec, board);


    let mut result = [true; 9];
    for i in 0..8{
        result[i] = column_eval[i] && row_eval[i] && square_eval[i];
    }

    result
}


// returns an array of booleans, the index is the possible number(-1) and true means possible valid, false means invalid
fn evaluate_by_column(tile_vec: (i8, i8), board: &Board) -> [bool; 9]{

    let column = board._board[tile_vec.0 as usize]; // 0=x 1=y

    let mut checking_array = [true; 9];


    // ideally this will always work, although there isnt any guess work to find incorect options
    for element in column.iter(){
        if(element.found){
            checking_array[element.final_guess as usize] = false;
        }
    }


    checking_array
}

fn evaluate_by_row(tile_vec: (i8, i8), board: &Board) -> [bool; 9]{

    let mut row: [Tile; 9] = [Tile::new_empty(); 9];

    for i in 0..8{
        row[i as usize] = board._board[i as usize][tile_vec.1 as usize];
    }

    let mut checking_array = [true; 9];

    for element in row.iter(){
        if(element.found){
            checking_array[element.final_guess as usize] = false;
        }
    }

    checking_array
}

fn evaluate_square(tile_vec: (i8, i8), board: &Board) -> [bool;9]{ // returns if a valid square

    let _board = board._board;

    let offset_x = tile_vec.0 % 3; // using the modulous tells us the offset to the origion(top left)
    let offset_y = tile_vec.1 % 3;

    let origion_pos_x = tile_vec.0 - offset_x;
    let origion_pos_y = tile_vec.1 - offset_y;

    let mut square: [Tile; 9] = [

        _board[origion_pos_x as usize][origion_pos_y as usize],
        _board[(origion_pos_x + 1) as usize][origion_pos_y as usize],
        _board[(origion_pos_x + 2) as usize][origion_pos_y as usize],
        _board[origion_pos_x as usize][(origion_pos_y + 1) as usize],
        _board[origion_pos_x as usize][(origion_pos_y + 2) as usize],
        _board[(origion_pos_x + 1) as usize][(origion_pos_y + 1) as usize],
        _board[(origion_pos_x + 1)as usize][(origion_pos_y + 2) as usize],
        _board[(origion_pos_x + 2) as usize][(origion_pos_y + 1) as usize],
        _board[(origion_pos_x + 2) as usize][(origion_pos_y + 2) as usize],

        //manually adds each tile into the array

    ];

    let mut checking_array : [bool; 9] = [true; 9];

    for element in square.iter(){
        if(element.found){
            checking_array[element.final_guess as usize] = false;
        }
    }


    checking_array
}