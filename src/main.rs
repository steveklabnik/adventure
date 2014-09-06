use std::io;

#[deriving(PartialEq)]
enum Command {
    North,
    South,
}

fn main() {
    println!("* * * A D V E N T U R E * * *\n\n");

    let mut command: Option<Command> = None;
    let mut stdin = io::stdin();

    while command == None {
        println!("You find yourself in a room. There is a door to the south.");
        println!("\nWhat do you do?\n");
        println!("* Go (s)outh");

        let input = stdin.read_line().ok().expect("Failed to read line");

        command = match input.as_slice().trim() {
            "s" => Some(South),
            _   => {
                println!("Please type a command.");
                continue;
            }
        };
    }

    match command.unwrap() {
        South => println!("Not implemented"),
        North => println!("Not implemented"),
    }

    command = None;

    while command == None {
        println!("You find yourself in a room. There is a door to the south, and a door to the north.");
        println!("\nWhat do you do?\n");
        println!("* Go (n)orth");
        println!("* Go (s)outh");

        let input = stdin.read_line().ok().expect("Failed to read line");

        command = match input.as_slice().trim() {
            "s" => Some(South),
            "n" => Some(North),
            _   => {
                println!("Please type a command.");
                continue;
            }
        };
    }

    match command.unwrap() {
        North => println!("Not implemented"),
        South => println!("Not implemented"),
    }

    command = None;
}
