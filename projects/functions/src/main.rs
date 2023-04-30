fn main() {
    println!("Hello, world!");
    another_function(five());
    print_labeled_measurement(5, 'h');
    let x = five();
    println!("function five expression: {x}"); 
    let y = plus_twofiftysix(x); 
    println!("5 plus 256 equals: {y}");
}

fn another_function(x: u8) {
    println!("another function!");
    println!("the value of x is {x}");
}

fn print_labeled_measurement(value: u32, unit_label: char) {
    println!("the measurement is {value}{unit_label}");
}

fn five() -> u8 {
    5
}

fn plus_twofiftysix(x: u8) -> u16 {
   (x + 256).into()
}
