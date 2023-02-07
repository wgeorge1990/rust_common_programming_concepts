use std::sync::mpsc;
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

    // using_move();

    let (tx, rx) = mpsc::channel();
    //Cloning the sender, mpsc = multiple sender and single reciever channel
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        // let val = String::from("hi");
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        // let val = String::from("hi");
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("another"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }


    });

    for received in rx {
        println!("GOT: {}", received)
    }

    // tx.send(val).unwrap();
    // below line will break and show borrow error at compile time
    // thanks to rusts ownership rules. This helps use write safe code
    // println!("val is {}", val);

    // let recieved = rx.recv().unwrap();
    // println!("Got: {}", recieved);
}

fn using_move() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
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
