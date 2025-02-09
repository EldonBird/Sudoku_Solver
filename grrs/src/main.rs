// use std::any::Any;
// use std::collections::HashSet;
// use std::io::empty;
// use std::env;


#[derive(Copy, Clone)]
struct Tile{
    found: bool,
    final_guess: i8, // 0-8, if you add 1, you would get what is expected from a sudoku board.
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

        if fg < 1 || fg > 9{
            panic!("Invalid value");
        }

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

        let mut found_index = -1;
        let mut count = 0;

        for (index, &value) in t.possible.iter().enumerate() {
            if value {
                found_index = index as i8 + 1;
                count += 1;
            }
        }

        if count == 1 {
            found_index
        } else {
            -1
        }

    }

    fn ready_to_guess(t: &Tile) -> bool{ // is the checking method to find if you can make a guess

        let mut iterator: i8 = 0;

        for i in t.possible.iter(){
            if *i{
                iterator += 1;
            }
        }

        if iterator == 1{
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

    fn new(input: &Board) -> Self {

        Board{
            _board: input._board,
        }
    }

    fn finished(input: &Board) -> bool{ // tells you if the board is completed

        for x in 0..9{
            for y in 0..9{

                if !input._board[x][y].found{
                    return false;
                }
            }
        }
        true
    }


}


fn main() {

    let board = read_in_board("Sudoku1.txt");

    println!("Input Board, ");


    output_board(&board);

    let fin = sudoku(board, 0);

    println!("Final Board, iterations : {}", fin.1);

    output_board(&fin.0);
}

fn sudoku(board: Board, current: i8) -> (Board, i8){
    let evaluation: Board= evaluate_whole_board(&board);
    let collapsed: Board = collapse(evaluation);


    if Board::finished(&collapsed){
        return (collapsed, current);
    }

    if board_comparison(&board, &collapsed){
        println!("Whoops Boards the same");
        return (collapsed, current);
    }


    sudoku(collapsed, current+1)
}

fn board_comparison(board1: &Board, board2: &Board) -> bool{

    for x in 0..9 {
        for y in 0..9 {
            if board1._board[x][y].final_guess != board2._board[x][y].final_guess
                || board1._board[x][y].found != board2._board[x][y].found {
                return false;
            }
        }
    }
    true

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

fn test_output_board(board: &Board){

    for x in 0..9{
        for y in 0..9{
            print!("{}", board._board[x][y].final_guess);
        }
    }
}
fn output_board(board: &Board){

    for x in 0..9 {
        if x > 0 {
            println!();
        }

        for y in 0..9 {
            print!("{} ", board._board[x][y].final_guess);

            if y % 3 == 2 && y < 8 {
                print!("| ");
            }
        }
    }
    println!();
}


fn evaluate_whole_board(input: &Board) -> Board{

    let mut _output: Board = Board::new(input);

    for x in 0..9{
        for y in 0..9{

            if input._board[x as usize][y as usize].found{
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

    for x in 0..9{
        for y in 0..9{

            if input._board[x as usize][y as usize].found {
                _output._board[x as usize][y as usize] = input._board[x as usize][y as usize];
            }
            else if Tile::ready_to_guess(&input._board[x as usize][y as usize]) { // if there is only 1 possible solution, then why not say that it is true!


                // THIS IS THE INDEX!!!!
                let guess: i8 = Tile::lock_in_guess(&input._board[x as usize][y as usize]);

                if guess >= 0{
                    _output._board[x as usize][y as usize] = Tile::new_found(guess);
                }
                else{
                    _output._board[x as usize][y as usize] = input._board[x as usize][y as usize];
                }
            }
            else{
                _output._board[x as usize][y as usize] = input._board[x as usize][y as usize];
            }
        }
    }


    _output
}


fn evaluate_tile(tile_vec: (i8, i8), board: &Board) -> [bool; 9] {

    let column_eval = evaluate_by_column(tile_vec, board);
    let row_eval = evaluate_by_row(tile_vec, board);
    let square_eval = evaluate_square(tile_vec, board);


    let mut result = [true; 9];
    for i in 0..9{
        result[i] = column_eval[i] && row_eval[i] && square_eval[i];
    }

    result
}


// returns an array of booleans, the index is the possible number(-1) and true means possible valid, false means invalid
fn evaluate_by_column(tile_vec: (i8, i8), board: &Board) -> [bool; 9]{

    let mut checking_array = [true; 9];


    for y in 0..9{

        if board._board[tile_vec.0 as usize][y].found{
            let guess = board._board[tile_vec.0 as usize][y].final_guess;
            if guess > 0 && guess <= 9{
                checking_array[(guess - 1) as usize] = false;
            }
        }
    }

    checking_array
}

fn evaluate_by_row(tile_vec: (i8, i8), board: &Board) -> [bool; 9]{

    let mut checking_array = [true; 9];

    for x in 0..9{
        if board._board[x][tile_vec.1 as usize].found{
            let guess = board._board[x][tile_vec.1 as usize].final_guess;
            if guess > 0 && guess <= 9{
                checking_array[(guess - 1) as usize] = false;
            }
        }
    }

    checking_array
}

fn evaluate_square(tile_vec: (i8, i8), board: &Board) -> [bool;9]{ // returns if a valid square

    let origin_x = (tile_vec.0 / 3) * 3;
    let origin_y = (tile_vec.1 / 3) * 3;

    let mut checking_array = [true; 9];

    for dx in 0..3 {
        for dy in 0..3 {
            let x = (origin_x + dx) as usize;
            let y = (origin_y + dy) as usize;



            if board._board[x][y].found {
                let guess = board._board[x][y].final_guess;
                if guess > 0 && guess <= 9 {
                    checking_array[(guess - 1) as usize] = false;
                }
            }
        }
    }

    checking_array
}