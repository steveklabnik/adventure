use std::io;

#[deriving(PartialEq)]
enum Command {
    Go(Direction),
}

#[deriving(PartialEq)]
enum Direction {
    North,
    South,
}

struct Exit {
    direction: Direction,
    target: u32, // the room number
}

struct Room {
    exits: Vec<Exit>,
}

fn main() {
    let rooms = vec![
        Room {
            exits: vec![
                Exit {
                    direction: South,
                    target: 1,
                },
            ],
        },
        Room {
            exits: vec![
                Exit {
                    direction: South,
                    target: 2,
                },
                Exit {
                    direction: North,
                    target: 0,
                },
            ],
        },
    ];

    println!("* * * A D V E N T U R E * * *\n\n");

    let mut command: Option<Command> = None;
    let mut stdin = io::stdin();

    while command == None {
        println!("You find yourself in a room. There is a door to the south.");
        println!("\nWhat do you do?\n");
        println!("* Go (s)outh");

        let input = stdin.read_line().ok().expect("Failed to read line");

        command = match input.as_slice().trim() {
            "s" => Some(Go(South)),
            _   => {
                println!("Please type a command.");
                continue;
            }
        };
    }

    match command.unwrap() {
        Go(North) => println!("Not implemented"),
        Go(South) => println!("Not implemented"),
    }

    command = None;

    while command == None {
        println!("You find yourself in a room. There is a door to the south, and a door to the north.");
        println!("\nWhat do you do?\n");
        println!("* Go (n)orth");
        println!("* Go (s)outh");

        let input = stdin.read_line().ok().expect("Failed to read line");

        command = match input.as_slice().trim() {
            "s" => Some(Go(South)),
            "n" => Some(Go(North)),
            _   => {
                println!("Please type a command.");
                continue;
            }
        };
    }

    match command.unwrap() {
        Go(North) => println!("Not implemented"),
        Go(South) => println!("Not implemented"),
    }

    command = None;
}
