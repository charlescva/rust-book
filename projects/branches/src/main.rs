
// https://doc.rust-lang.org/book/ch03-05-control-flow.html

// An if expression allows you to branch your code's flow depending on conditions.

// You provide a condition and then state, 

fn main() {
    let number = 7;

// “If this condition is met, run this block of code. 
    if number < 5 {
        println!("condition was true");

// If the condition is not met, do not run this block of code.”
    } else {
        println!("condition was false");
        modulo();
    }
}

// Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions.


// This program has four possible paths it can take
fn modulo() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    if_is_an_expression();

}

// Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable.

fn if_is_an_expression() {
    let condition = true;
    let number = if condition { 1 } else { 0 };

    println!("The value of the number is: {number}");
}

// values that have the potential to be results from each arm of the 'if' must be the same type; above, the results of both the 'if' arm and the 'else' arm were i32 types. If the types are mismatched, we’ll get an error.
