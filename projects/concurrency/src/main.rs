use std::thread;
use std::time::Duration;

fn main() {

    // The return type of thread::spawn is JoinHandle. 
    // A JoinHandle is an owned value 
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });


    // a call to handle.join() here would cause the main thread to wait for the spawned thread to
    // finish before continuing.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // when we call the join method on it, will wait for its thread to finish. 
    handle.join().unwrap();
    
    println!("Fearless Concurrency!");
    //https://doc.rust-lang.org/book/ch16-01-threads.html
   

    // in this example, the closure takes ownership of value 'v' using the 'move' keyword
    let v = vec![1, 2, 3];
    let vhandle = thread::spawn(move|| {
        println!("Here is a vector: {:?}", v);
    });

    vhandle.join().unwrap();
}
