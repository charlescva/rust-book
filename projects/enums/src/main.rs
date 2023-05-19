enum TcpService {
    SSH(u32),
    HTTPS(u32),
    DNS(u32),
}

enum ShapeKind {
    SQUARE(u32, u32), // height, width
    CIRCLE(u32), // radius
}

fn main() {

    // Listing 6-1: Storing the data and IpAddrKind variant of an IP 
    // address using a struct

    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let webService = TcpService::HTTPS(443);
    let shellService = TcpService::SSH(22);
    let square = ShapeKind::SQUARE(10, 30);

    fn route(ip_kind: IpAddrKind) {}
    sendMessage();
}


// Listing 6-2: A Message enum whose variants each store different
// amounts and types of values.

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),

    // just as we’re able to define methods on structs using impl, 
    // we’re also able to define methods on enums.
    
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("message!");
    }
}

// if we used the different structs, each of which has its own type, 
// we couldn’t as easily define a function to take any of these kinds 
// of messages as we could with the Message enum


fn sendMessage() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn options() {
    let some_number = Some(5);
    let some_char = Some('c');

    let absent_number: Option<i32> = None;
}


