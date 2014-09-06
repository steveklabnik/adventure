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

impl Room {
    fn can_go(&self, direction: Direction) -> bool {
        self.exits.iter().find(|e| e.direction == direction).is_some()
    }

    fn exit_to(&self, direction: Direction) -> u32 {
        self.exits.iter()
                  .find(|e| e.direction == direction)
                  .unwrap()
                  .target
    }
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

    let mut current_room = 0;

    println!("* * * A D V E N T U R E * * *\n\n");

    let mut command: Option<Command> = None;
    let mut stdin = io::stdin();

    while current_room != 2 { // hard code the exit for now...
        current_room = enter(&rooms[current_room]);
    }

}

fn enter(room: &Room) -> uint {
    let mut command: Option<Command> = None;

    while command == None {
        println!("You find yourself in a room. There is a door to the south.");
        println!("\nWhat do you do?\n");

        for exit in room.exits.iter() {
            match exit.direction {
                North => println!("* Go (n)orth"),
                South => println!("* Go (s)outh"),
            }
        }

        let input = io::stdin().read_line().ok().expect("Failed to read line");

        command = match input.as_slice().trim() {
            "n" if room.can_go(North) => Some(Go(North)),
            "s" if room.can_go(South) => Some(Go(South)),
            _   => {
                println!("Please type a valid command.");
                continue;
            }
        };
    }

    let next_room = match command.unwrap() {
        Go(North) => room.exit_to(North),
        Go(South) => room.exit_to(South),
    };

    next_room as uint
}
