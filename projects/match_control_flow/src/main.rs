/*
 * Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things; Chapter 18 covers all the different kinds of patterns and what they do. The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.
 */

// Listing 6-3: An enum and a match expression that has the variants of the enum as its patterns
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Listing 6-4: A Coin enum in which the Quarter variant also holds a UsState value
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    println!("Hello, match control flow!");

    println!("A Nickel is worth {} cents!", value_in_cents(Coin::Nickel));
    println!("See a penny pick it up, and all the day you'll have good...{}", value_in_cents(Coin::Penny));
    println!("An Alaskan Quarter is worth {} cents!", value_in_cents(Coin::Quarter(UsState::Alaska)));
}

fn value_in_cents(coin: Coin) -> u8 {

    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }

}
