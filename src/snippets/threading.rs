use crossbeam_channel::{bounded, unbounded};
use std::sync::mpsc;
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;

extern crate crossbeam_channel;

/// Code snippets for dealing with threads in Rust
/// Most of them are slightly modified version of
/// documentation samples.
/// The point is to use this file as a reference for
/// any usage of threading.
fn main() {
    /////////////////////////////////////////////
    // Thread with return value and simple
    // synchronization
    {
        let handler = thread::spawn(|| "Hello world");
        let result = handler.join();

        // Print Hello world
        assert_eq!("Hello world", result.unwrap());
    }

    /////////////////////////////////////////////
    // Panic
    {
        #[allow(unreachable_code)]
        let handler = thread::spawn(|| {
            panic!("oops!");

            // Unreachable code
            let result_val = 10;
            result_val
        });

        let result = handler.join();
        if result.is_err() {
            print!("Thread failed");
        }
    }

    /////////////////////////////////////////////
    // Threads do not borrow

    let name = "dolly".to_string();
    // capture values, but by moving, not by borrowing
    let t = thread::spawn(move || {
        println!("hello {}", name);
    });
    println!("wait {:?}", t.join());

    // Expected error:
    // print!("{}",name);
    //              ^^^^ value used here after move

    /////////////////////////////////////////////
    // An asynchronous, infinitely buffered channel
    {
        // Allocate asynchronous,
        // infinitely buffered channel.

        let (tx1, rx) = mpsc::channel();

        // 10 OS Threads running calculation
        // and sending result to the main thread
        for i in 1..10 {
            let tx = tx1.clone();

            thread::spawn(move || {
                let answer = i * 10;
                print!("\nThread num: {}", i);
                let _r = tx.send(answer);

                thread::sleep(std::time::Duration::from_millis(500));
            });
        }

        drop(tx1);

        /* Option one to pull from the buffer up to 10 msgs
        for _ in 0 .. 10 {
            // Attempts to wait for a value on this receiver,
            // returning an error if the corresponding channel has hung up
            let result = rx.recv();
             print!("\nReceived: {}",result.unwrap());
        }
        */

        //  Another way to pull msgs from the channel
        //  loop ends when the channel is dropped.
        //  Chennel is dropped when all threads release tx
        let mut cksum = 0;
        for result_value in rx {
            cksum += result_value;
        }

        // assert if all recieved
        assert_eq!(cksum, 450);
    }

    /////////////////////////////////////////////
    // A synchronous, bounded channel
    // Similar as buffered channels in Golang
    {
        // buffered channel with 1 slot
        let (sender, receiver) = sync_channel(1);

        // first slot. Channel buffer has not other free slots
        // So, all other sends will block until recv is called
        sender.send(1).unwrap();

        // If uncomment main thread will hang
        // sender.send(2).unwrap();

        thread::spawn(move || {
            sender.send(2).unwrap();
        });

        let mut cksum = 0;
        for result_value in receiver {
            cksum += result_value;
        }

        assert_eq!(cksum, 3);
    }

    /////////////////////////////////////////////
    // Thread attributes
    {
        // Assign Thread name
        let builder = thread::Builder::new().name("MyThread".to_string());

        let _ = builder
            .spawn(|| assert_eq!(thread::current().name(), Some("MyThread")))
            .unwrap();

        // Assign Thread name + stack size change
        let builder = thread::Builder::new()
            .name("MyThread2".to_string())
            .stack_size(32 * 1024);

        // Spawn a thread that allocates stack mem bigger that the set limit
        let handler = builder
            .spawn(|| {
                // Force stack overflow
                // let v1: [char; 32 * 1024] = [0 as char; 32 * 1024];
                let _v1: [char; 1 * 1024] = [0 as char; 1 * 1024];
            })
            .unwrap();

        // if v1 size is bigger than (20 * 1024) , as expected we get this err:
        //
        // thread 'MyThread2' has overflowed its stack
        // fatal runtime error: stack overflow
        // Aborted (core dumped)
        // Seems we cannot fail gracefully in that situation

        let result = handler.join();
        if result.is_err() {
            print!("Thread failed");
        }
    }

    /////////////////////////////////////////////
    // Shared mutable data between threads using
    // mutex
    {
        struct Node {
            value: u64,
            name: String,
        }

        let protected_value: Node = Node {
            value: 111111,
            name: "Name".to_string(),
        };

        let counter = Arc::new(Mutex::new(protected_value));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = counter.clone();
            let handle = thread::spawn(move || {
                let mut node = counter.lock().unwrap();
                node.value += 1;
                node.name = "some_name".to_string();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.lock().unwrap().value, 111121);
    }

    // crossbeam_channel.
    // Seems to be a crate of out of std that is
    // preferable for message passing between threads.
    // Better perf and features set.
    //
    // Multi-producer multi-consumer channels for message passing.
    // https://docs.rs/crossbeam-channel/0.3.6/crossbeam_channel/
    {
        // Create a channel of unbounded capacity.
        let (s, r) = unbounded();

        // Send a message into the channel.
        s.send("Hello, world!").unwrap();

        // Receive the message from the channel.
        assert_eq!(r.recv(), Ok("Hello2, world!"));
    }

    // crossbeam_channel
    // Senders and receivers can be cloned and sent to other threads:
    {
        let (s1, r1) = bounded(0);
        let (s2, r2) = (s1.clone(), r1.clone());

        // Spawn a thread that receives a message and then sends one.
        thread::spawn(move || {
            r2.recv().unwrap();
            s2.send(2).unwrap();
        });

        // Send a message and then receive one.
        s1.send(1).unwrap();
        r1.recv().unwrap();
    }
}
