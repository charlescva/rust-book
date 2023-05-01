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

    // Listing 5-3: Creating an instance of the User struct

    let mut user1 = User {
        active: true,
        username: String::from("failboat"),
        email: String::from("failboat@sadc2.org"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@sadc2.org");


}

// Listing 5-4: A build_user fn that takes an email and username
// and returns a User instance

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }

}
