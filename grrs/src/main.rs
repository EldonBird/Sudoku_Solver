use std::io::empty;

struct Tile{
    found: i8, // for testing, but the found solution
    possible: [i8; 0], // all possible solutions
    position: (i8, i8), // x, y
}

struct Square{
    tiles: [Tile; 9],
}


fn main() {
    println!("Hello, world!");
}



fn EvaluateBoard() -> [[i8; 9]; 9] {


    return null;
}

fn EvaluateTile() -> [i8; 9] {




    return null;
}

fn EvaluateByRow(tile: Tile){




}

fn EvaluateByColumn(tile: Tile){



}

fn EvaluateSquare(){ // returns if a valid square




}


fn AvailableByRow(){ // returns remaining possible numbers in a row




}