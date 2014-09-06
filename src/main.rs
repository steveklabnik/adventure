use std::io;

#[deriving(PartialEq)]
enum Command {
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

}
