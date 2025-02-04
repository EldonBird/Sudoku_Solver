use std::any::Any;
use std::io::empty;

struct Tile{
    found: bool,
    final_guess: i8, // for testing, but the found solution
    possible: [bool; 9], // all possible solutions, true if available, false if not 0 = 1, so 8 = 9
    position: (i8, i8), // x, y


}

struct Square{
    tiles: [Tile; 9],
}


fn main() {
    println!("Hello, world!");
}



fn EvaluateBoard() -> [[Tile; 9]; 9] {


    return null;
}

fn EvaluateTile(tile: Tile, board: [[Tile; 9]; 9]) -> [bool; 9] {

    let Xpos = tile.position.0;


    return null;
}


// returns an array of booleans, the index is the possible number(-1) and true means possible valid, false means invalid
fn EvaluateByRow(tile: Tile, board: [[Tile; 9]; 9]) -> [bool; 9]{

    let row = board[tile.position.1]; // 0=x 1=y

    let mut checking_array = [true; 9];


    // ideally this will always work, although there isnt any guess work to find incorect options
    for element in row.iter(){
        if(element.found){
            checking_array[element.final_guess] = false;
        }
    }


    return checking_array;
}

fn EvaluateByColumn(tile: Tile, board: [[Tile; 9]; 9]) -> [bool; 9]{

    let mut column: [Tile; 9] = [Tile::defualt(); 9];
    let xpos = tile.position.0; // 0=x 1=y

    for i in 0..8{
        column[i] = board[i][xpos];
    }

    let mut checking_array = [true; 9];

    for element in column.iter(){
        if(element.found){
            checking_array[element.final_guess] = false;
        }
    }


    return checking_array;
}

fn EvaluateSquare(){ // returns if a valid square




}