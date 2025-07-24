use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "the letter e";
    let mut trials = 0;

    loop {
        println!("{}", riddle);
        trials += 1;

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.trim().to_lowercase() == answer {
            println!("Number of trials: {}", trials);
            break;
        }
    }
}