use std::io;

fn my_fib(nth: u32) -> u32 {
    match nth {
        1 => 0,
        2 => 1 + my_fib(nth-1),
        _ => my_fib(nth - 2) + my_fib(nth - 1),
    }  
}

fn my_iter_fib(nth: u32) -> u32 {
    let (mut x, mut y, mut z) = (0u32, 1u32, 0u32);
    for number in 0..nth {
        if number > 1 {
            z = x + y;
            x = y;
            y = z;
        }
    }
    if nth == 0 {
        x
    } else {
        y
    }
}

fn main() {
    let funfun = my_iter_fib(6);
    println!("{}", funfun);
    loop {
        println!("(0 to exit) N_th Fibonacci number: ");
        let mut nth = String::new();
        io::stdin().read_line(&mut nth)
          .expect("Failed to read line");
        let nth: u32 = match nth.trim().parse() {
            Ok(n) => {
                if n == 0 { break }
                else { n }
            },
            Err(_) => {
                println!("Must enter an integer");
                continue;
            }
        };
        let fib = my_fib(nth);
        println!("The {}th Fibonacci number is {}.", nth, fib);
    }

}
