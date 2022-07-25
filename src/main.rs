use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    variables()
}

fn variables() {
    let mut x = 5
    println!("x is {x}");
    {
        let x = 54;
        println!("x is {x}")
    }
    println!("x is {x}")
}

fn number_guesser() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guessxd");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed lol");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("TOo Big"),
            Ordering::Equal => {
                println!("DING DING DING CORRECT");
                break;
            }
        }
    }
}
