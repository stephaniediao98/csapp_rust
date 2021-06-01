# 12.4 &nbsp; Shared Variables in Threaded Programs
One of the main benefits of using threads is the ability to easily share the same program variables with other threads. However, sharing memory between multiple threads that run at the same time can be tricky and dangerouse. Recall from the Introduction of this chapter that this can cause race conditions and deadlocks. Fortunately, Rust's fearless concurrency makes sharing data much less intimidating and unsafe.

### 12.4.1 &nbsp; Using `move` Closures with Threads
The `move` closure can be used alongside `thread::spawn` to allow you to use data from one thread in another. To use data from the main thread in a spawned thread, the spawned thread's closure must capture the values it needs. This can be done using `Closure`s. 

`Closure`s need to be used with `move` so that `Closure`s are forced to take ownership of the values they are using rather than borrowing them, which the Rust compiler automatically infers if `move` is not used. Let's try an example where we give the main thread a `String` that we want to share with a peer thread:

```rust, editable
use std::thread;

fn main() {
    let shared_string = String::from("fearless concurrency!");

    let handle = thread::spawn(move || {
        println!("What's one of Rust's coolest features? {}", shared_string);
    });

    handle.join().unwrap()
}
```
Running the example shows the expected ouput: `What's one of Rust's coolest features? fearless concurrency!`. Now, consider the following questions: 

* What would have happened if `move` was not used? If you're unsure, try it out by editing the code block above! 
* What error message do you get when you remove `move`? What does it mean? How does it relate to ownership and borrowing? 

### 12.4.2 &nbsp; Communication Among Threads
Threads can communicate via **message passing**, by which they send each other messages containing data. Rust implements `channel`s, which are used to facilitate message-sending concurrency. Channels are similar to streams or rivers because when a boat or rubber duck enters the stream, it travels downstream until it reaches the end of the water. In programming, channels have two main components: a **transmitter** and a **receiver**. The transmitter is in the upstream location where rubber ducks and boats enter the stream. The receiver is in the downstream location where the rubber ducks and boats end up after traveling. In your code, the transmitter holds the data you want to send, while the receiver checks for arriving messages. One part of your code is dedicated to the transmitter while the other part is dedicated to the receiver. Let's get familiar with Rust's `channel` by going through an example, step-by-step:

```rust, ignore
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```
The block of code above creates a new channel between a transmitter and a receiver (`tx` and `rx` are the traditionally used names for transmitters and receivers, respectively). It does so using the `mpsc::channel` function, where `mpsc` stands for **multiple producers, single consumer**. Please refer to the `Aside` below for more information, but the general idea of `mpsc` is that a channel can have multiple transmitters that send values and only one consumer that receives those values. 

>**Aside** Producer-Consumer Problem

Now, let's try getting the transmitter end of the channel working. We can begin by moving the transmitter into a spawned thread. The spawned thread will then be communicating with the main thread via the channel. 

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let message = String::from("hi receiver! it's me, transmitter.");
        tx.send(message).unwrap();
    })
}
```
[`send`](https://doc.rust-lang.org/std/marker/trait.Send.html) is used by transmitters. It returns a `Result<T, E>` type.