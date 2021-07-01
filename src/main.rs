use std::io;

fn main() {
    // Variables
    let mut gamecontinues = true; // Flag for the game loop
    //let mut winner: u8;
    let mut table: [char;9] = ['0','1','2','3','4','5','6','7','8']; // An array doesn't need to be mutable!!
    let mut player: u32 = 0; // Switching between P1 and P2

    while gamecontinues == true {
        showtable (&mut table[..]); // Prints the table
        gamecontinues = playerposition(&mut table, &mut player);
    }
}

fn showtable(t: &mut [char]) {
    for (i, &v) in t.iter().enumerate() {  // i is the index, &v is the char value
        print!("{}    ", v);
        if i == 2 || i == 5 || i == 8 {
            println!();
            println!();
        }
    }
}


fn playerposition(t: &mut [char], p: &mut u32) -> bool {
    let mut input = String::new(); // Input from stdin
    let pos: usize; // Postion chosen by the player
    let mut s: char = 'O';
    let remainder: u32;

    remainder = *p % 2; // The remainder is used to switch player

    if remainder == 0{
        println!("Player 1, choose a box: ");
        s = 'X';
        *p +=1;   // To assign or do operations with a reference of a mutable value it must be dereferenced with *.
        
    } else if remainder == 1 {
        println!("Player 2, choose a box: ");
        s = 'O';
        *p +=1;
    }

    // Input a character (String)
    io::stdin().read_line(&mut input).expect("Failed to read string");

    pos = input.trim().parse().unwrap(); // unwrap() returns the value from the string slice; trim() converts a String into a &str.
    
    match pos {
        0 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        1 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        2 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        3 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        4 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        5 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        6 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        7 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        8 => { if t[pos] != 'X' || t[pos] != 'O' {t[pos] = s;} else {println!("The box is occupied!!");} },
        _ => panic!("Unexpected position!!"),
    }

    let game = positioncheck(t);

    return game;
}


fn positioncheck(t: &mut [char]) -> bool{
    
    let mut continua: bool = true ;

    if t[0] == 'X' && t[1] == 'X' && t[2] == 'X' {          // P1 wins
        println!("Player 1 wins!!");
        continua = false;
    } else if t[3] == 'X' && t[4] == 'X' && t[5] == 'X' {
        println!("Player 1 wins!!");
        continua = false;
    } else if t[6] == 'X' && t[7] == 'X' && t[8] == 'X' {
        println!("Player 1 wins!!");
        continua = false;
    } else if t[0] == 'X' && t[3] == 'X' && t[6] == 'X' {
        println!("Player 1 wins!!");
        continua = false;
    } else if t[1] == 'X' && t[4] == 'X' && t[7] == 'X' {
        println!("Player 1 wins!!");
        continua = false;
    } else if t[2] == 'X' && t[5] == 'X' && t[8] == 'X' {
        println!("Player 1 wins!!");
        continua = false;
    } else if t[0] == 'X' && t[4] == 'X' && t[8] == 'X' {
        println!("Player 1 wins!!");
        continua = false;
    } else if t[2] == 'X' && t[4] == 'X' && t[6] == 'X' {
        println!("Player 1 wins!!");
        continua = false;
    } else if t[0] == 'O' && t[1] == 'O' && t[2] == 'O' {   // P2 wins
        println!("Player 2 wins!!");
        continua = false;
    } else if t[3] == 'O' && t[4] == 'O' && t[5] == 'O' {
        println!("Player 2 wins!!");
        continua = false;
    } else if t[6] == 'O' && t[7] == 'O' && t[8] == 'O' {
        println!("Player 2 wins!!");
        continua = false;
    } else if t[0] == 'O' && t[3] == 'O' && t[6] == 'O'{
        println!("Player 2 wins!!");
        continua = false;
    } else if t[1] == 'O' && t[4] == 'O' && t[7] == 'O' {
        println!("Player 2 wins!!");
        continua = false;
    } else if t[2] == 'O' && t[5] == 'O' && t[8] == 'O' {
        println!("Player 2 wins!!");
        continua = false;
    } else if t[0] == 'O' && t[4] == 'O' && t[8] == 'O' {
        println!("Player 2 wins!!");
        continua = false;
    } else if t[2] == 'O' && t[4] == 'O' && t[6] == 'O' {
        println!("Player 2 wins!!");
        continua = false;
    }

    continua
}