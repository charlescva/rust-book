fn main() {
    //In mathematics, the Fibonacci sequence is a sequence in which
    //each number is the sum of the two preceding ones.
    
    println!("Hello, Fibonacci!");
  
    // The nth Number in a sequence...
    let n: u32 = 32;

    // A tuple to store the two preceding values.
    let mut tup: (u32, u32) = (0, 1);
  
    // Loop Counter
    let mut count = 1;
    loop {
        let sum = tup.0 + tup.1; 
        tup.0 = tup.1;
        tup.1 = sum;
        println!("Fibonacci Sequence #{count} is  {sum}.");
        if count == n {
            break;
        }
        count += 1;
    }
}
