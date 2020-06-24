use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let answer = rand::thread_rng().gen_range(1, 101);
    //let answ_range = Uniform::new_inclusive(1,100);
    //let answer = answ_range.sample(&mut rng);

    println!("Hello, world!");
    println!("I'm making a guessing game.");
    loop {
        println!("Guess a number (1-100):");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        
        let guess :u32 = match guess
                          .trim()
                          .parse() {
                              Ok(num) => num,
                              Err(_) => {
                                  println!("Must enter an integer. Try again.");
                                  continue;
                              }
                          };
        
        println!("Your guess was {}.", guess);

        if guess == 0 {
            println!("You gave up. The answer was {}.", answer);
            break;
        }
        
        match guess.cmp(&answer) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("Correctomundo!");
                break;
            }
        }

    }
}
