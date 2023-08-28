use std::io;
use rand::prelude::*;


fn main() {
    let rand_numb = thread_rng().gen_range(1..101);
    let mut guess = String::new();
    let mut number_guess: i32 = 0;

    println!("Test answer is {rand_numb}");

    while number_guess != rand_numb {
        guess.replace_range(.., "");
        println!("What is your guess?: ");
        io::stdin().read_line(&mut guess);

        println!("your guess is {guess}");
        number_guess = guess.trim().parse().unwrap();
        if number_guess > rand_numb {
            println!("{number_guess} is too high");
        } else if number_guess < rand_numb {
            println!("{number_guess} is too low");
        }

    }

    println!("{number_guess} is correct");
    

}
