use std::thread;
use std::time::Duration;

fn main() {
    println!("creating thread and printing to a spawned thread");
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hello number {} from spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    
    // for i in 1..5 {
    //     println!("hello number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // using_join();
    using_move();
}

fn using_move() {
    let v: Vec<i32> = vec![1,2,3,4,5];

    let handle = thread::spawn(move|| {
        for n in v {
            println!("{}", n);
        }
    });

    handle.join().unwrap();
}

fn using_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
