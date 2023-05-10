// Listing 5-1: Creating a User struct definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Listing 5-10: Defining a Rectangle struct
//Listing 5-12: Adding the attribute to derive the Debug trait
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
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

    let scale = 2;
    // Listing 5-10 - Declaring and instance of Rectangle struct
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // Listng 5-11: Attemping to print Rectangle instance
    println!("rect 1 is {:?}", rect1);

    // debug prints to stderr instead of stdout
    dbg!(&rect1);

    // Listing 5-10 - Passing Reference to fn area
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
        );

}

// Listing 5-10 - expression returns area using the . separator
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
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
