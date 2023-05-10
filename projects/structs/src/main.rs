// Listing 5-1: Creating a User struct definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, Structs!");

    // Listing 5-2: Creating an instance of the User struct
    let user1 = User {
        active: true,
        username: String::from("failboat"),
        email: String::from("failboat@sadc2.org"),
        sign_in_count: 1,
    };

    println!("Created a new user named, {}",user1.username);
    // Listing 5-3: Creating an instance of the User struct

    let mut user1 = User {
        active: true,
        username: String::from("failboat"),
        email: String::from("failboat@sadc2.org"),
        sign_in_count: 1,
    };
    println!("Mutable User's initial email, {}", user1.email);

    user1.email = String::from("anotheremail@sadc2.org");

    println!("Mutable user's new email is, {}", user1.email);

    // Listing 5-7: Using struct update syntax to set email
    let user2 = User {
        email: String::from("anotheruser@example.com"),
        ..user1
    };

    println!("This user was created with struct update syntax!\r\n{}", user2.username);

    println!("Building a new user...");
    let user3 = build_user(String::from("functionaluser@gmail.com"), String::from("funcy_user"));
    println!("Signin Count: {}, Status: {}", user3.sign_in_count, user3.active);
}

// Listing 5-4: A build_user fn that takes an email and username
// and returns a User instance

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email, // Field Init Shorthand
        sign_in_count: 1,
    }

}
