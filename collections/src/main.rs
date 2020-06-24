use std::{collections::HashMap, io};

fn main() {
    println!("Here are some collection exercises:");

    let mut exercises: HashMap<String, fn() -> bool> = HashMap::new();
    // exercises holds titles and then functions
    exercises.insert(String::from("Integers"), integers);
    exercises.insert(String::from("Pig"), pig);
    exercises.insert(String::from("Directory"), directory);
    exercises.insert(String::from("Help"), help);

    let mut exec = true;
    while exec {
        println!("Pick a collection demo:");

        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)
          .expect("Failed to read line");
        cmd = cmd.trim().to_string();

        let program = match exercises.get(&cmd) {
            Some(exercise) => *exercise,
            None => quit
        };
        exec = program();
    }
}

fn quit() -> bool { false }

fn integers() -> bool {
    println!("Enter length of array:");

    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Error reading input");
    let size :u8 = match size.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Must enter an integer. Try again.");
            return false;
        }
    };

    let mut v = Vec::<i32>::new();
    let mut i :u8 = 0;
    while i < size {
        println!("Enter an integer: ");
        let mut int = String::new();

        match io::stdin().read_line(&mut int) {
            Ok(_) => {
                match int.trim().parse::<i32>() {
                    Ok(num) => v.push(num),
                    Err(_) => {
                        println!("Must enter an integer. Try again.");
                        continue;
                    }
                }
            },
            Err(error) => println!("Error reading input: {}", error)
        }
        i += 1;
    }

    let mut sum :i32 = 0;
    for val in &v {
        sum += val;
    }
    let avg: f32 = (sum as f32) / (size as f32);
    println!("Average of {:?} is {}", v, avg);
    true
}

fn pig() -> bool {
    println!("You're in Pig.");
    true
}
fn directory() -> bool {
    println!("You're in Directory.");
    true
}
fn help() -> bool {
    println!("You're in Help.");
    false
}