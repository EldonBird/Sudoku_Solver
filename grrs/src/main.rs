use std::any::Any;
use std::collections::HashSet;
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

fn GetIntersection(array1: [bool; 9], array2: [bool; 9]) -> [bool; 9]{

    let mut outputarray : [bool; 9] = [true; 9];

    let mut hashset: HashSet<bool> = HashSet::new();





    return outputarray;

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

fn EvaluateSquare(tile: Tile, Board: [[Tile; 9]; 9]) -> [bool;9]{ // returns if a valid square

    let offsetX = tile.position.0 % 3; // using the modulous tells us the offset to the origion(top left)
    let offsetY = tile.position.1 % 3;

    let origion_pos_x = tile.position.0 - offsetX;
    let origion_pos_y = tile.position.1 - offsetY;

    let mut square: [Tile; 9] = [

        Board[origion_pos_y][origion_pos_x],
        Board[origion_pos_y + 1][origion_pos_x],
        Board[origion_pos_y + 2][origion_pos_x],
        Board[origion_pos_y][origion_pos_x + 1],
        Board[origion_pos_y][origion_pos_x + 2],
        Board[origion_pos_y + 1][origion_pos_x + 1],
        Board[origion_pos_y + 1][origion_pos_x + 2],
        Board[origion_pos_y + 2][origion_pos_x + 1],
        Board[origion_pos_y + 2][origion_pos_x + 2],

        //manually adds each tile into the array

    ];

    let mut checking_array : [bool; 9] = [true; 9];

    for element in square.iter(){
        if(element.found){
            checking_array[element.final_guess] = false;
        }
    }


    return checking_array;
}