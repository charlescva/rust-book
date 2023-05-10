// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

fn main() {
    println!("Hello, Tuple Struct!");
//    let black = Color(0, 0, 0);
//    let origin = Point(0, 0, 0);


    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
        );

    let rec1 = (30, 50);
    println!(
        "The tuple defined rectangle is {} square pixels.",
        area_tuple(rec1)
        );


}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
