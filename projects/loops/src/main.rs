fn main() {
    // It's often useful to execute a block of code more than once.
    // There are THREE kinds of loops: loop, while, and for. 
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
 
    println!("The result is {result}");
    for_loop();
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0; 

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;

    }

    lift_off();
}

fn lift_off() {
    for number in (1..4).rev() {
        println!("{number}...");
    }
    println!("LIFTOFF!!!");
}
