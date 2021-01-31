extern crate rand;
extern crate colored;
use colored::*;

fn garbage() {

    let u = 0;

    println!(""); // make space between the lines
    println!("the value of u is {}", u);
}

fn predict() {

    println!(""); // make space between the lines
    println!("think a number and write it there");
    let mut usernumber = String::new();
    std::io::stdin().read_line(&mut usernumber).expect("bruh");
    
    let _err: i32 = usernumber.trim().parse().ok().expect("put a number :)");

    println!(""); // make space between the lines
    println!("the value you selected is {}", usernumber);
}

fn userloop() {

    println!(""); // make space between the lines
    println!("Think a number and wirte it there");
    let mut count = String::new();
    std::io::stdin().read_line(&mut count).expect("failed to read line");
    let mut _loopnum: i32 = count.trim().parse().ok().expect("that's not number bruh");
    
    loop {
        _loopnum += 1;

        println!("{}", _loopnum);
    
        // prevent my wsl from going out of RAM 
        if _loopnum >= 1000 {
            break;
        }
    }
}

fn calc() {

    println!(""); // make space between the lines

    println!("Enter the first number of the operation");
    let mut num1 = String::new();
    std::io::stdin().read_line(&mut num1).expect("failed to read line");

    println!(""); // make space between the lines

    println!("Enter the second number of the operation");
    let mut num2 = String::new();
    std::io::stdin().read_line(&mut num2).expect("failed to read line");

    println!(""); // make space between the lines

    let mut _num1: i32 = num1.trim().parse().ok().expect("that's not a number");
    let mut _num2: i32 = num2.trim().parse().ok().expect("that's not a number");

    println!("Sum result is: {}", _num1 + _num2);
    println!(""); // make space between the lines
    
    println!("Difference result is: {}", _num1 - _num2);
    println!(""); // make space between the lines

    println!("Moltiplication result is: {}", _num1 * _num2);
    println!(""); // make space between the lines

    println!("Division result is: {}", _num1 / _num2);
    println!(""); // make space between the lines
}

fn random_number_bool() {

    println!("select a number 1-2:
                        1 -> random number (from 0 to ∞)
                        2 -> random boolean value (true or false)
    ");
    println!(""); // make space between the lines

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let mut _choice: i32 = choice.trim().parse().ok().expect("that's not a valid number");

    let random: u8 = rand::random();

    let xbool: bool = rand::random();

    if _choice == 1 {
        println!("The random number is {}", random);

        println!(""); // make space between the lines
    } else if _choice == 2 {
        println!("The random bool is {}", xbool);

        println!(""); // make space between the lines
    }
}

fn main() {

    println!("{}", "Welcome to Crispy-Guacamole ^_^".cyan().bold());
    println!(""); // make space between the lines

    // help
    println!("{}","help section:
                    1 -> garbage (return 0, cause it's the value of x)
                    2 -> predict (magic, know the value you selected)
                    3 -> userloop (count from a number the user decide, until 1000)
                    4 -> calculator (supported operations: +, -, *, /)
                    5 -> random number or bool value (print a random number or a boolen value)
    ".yellow().bold());

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("failed to read line");
    let mut _choicenum: i32 = choice.trim().parse().ok().expect("that's not a valid number choice");
    
    if _choicenum == 1 {
        garbage();
    } else if _choicenum == 2 {
        predict();
    } else if _choicenum == 3 {
        userloop();
    } else if _choicenum == 4 {
        calc();
    } else if _choicenum == 5 {
        random_number_bool();
    } else {
        println!("You didn't selected any valid number (they're 1-2-3-4-5)");
    }

}