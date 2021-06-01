# 12.3: &nbsp; Concurrent Programming with Threads
Thus far, we have looked at two approaches to writing concurrent programs: using processes and using I/O multiplexing. In this section, we discuss a third approach using threads that is a hybrid of these two. We also discuss how to use Rust's [`std::thread`](https://doc.rust-lang.org/std/thread/) crate to perform concurrency. 

## 12.3.1 &nbsp; Threads
A **thread** is a logical flow that runs within the context of a process. Most modern operating systems run multiple threads concurrently in a single process. 

Just like processes, threads are scheduled automatically by the kernel. Furthermore, every thread has its own **thread context**, which includes its **thread ID (TID)**, stack, stack pointer, program counter, general-purpose registers, and condition codes. All threads that run within a process share the process's virtual address space, which includes its code, data, heap, shared libraries, and open files.

## 12.3.2 &nbsp; Thread Execution Model
TODO! [helpful link](https://doc.rust-lang.org/std/thread/)

## 12.3.3 &nbsp; Posix Threads
TODO!

## 12.3.4 &nbsp; Creating Threads
Rust provides the [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html) function for creating new threads. 

```rust, ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static, 
```

The `spawn` function creates a new thread and returns a [`JoinHandle`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html) for it. The `JoinHandle` struct represents an owned permission to join on a thread. It detaches the child thread when it is dropped, which means that there is no longer any handle to the thread and thus no way to [`join`](https://doc.rust-lang.org/std/thread/struct.JoinHandle.html#method.join) on it. The `join` method is implemented for `JoinHandle`s and is used to join the child thread. 

There are two constraints on both the closure argument `F` given to `spawn` and its return value `T`: 

* *Static:* The `'static` constraint indicates that the closure and its return value must have a lifetime of the whole program execution. Because threads can detach and outlive the lifetime they have been created in, we need to ensure that they are valid after they outlive their caller. However, since we don't know exactly how long they will be valid, we use the `'static` constraint to keep them valid as long as possible.
* *Send:* The `Send` trait is automatically implemented by the Rust compiler on types that can be transferred across thread boundaries. An example of a non-`Send` type is [`rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html), which is a single-threaded reference-counting pointer. This is a non-`Send` type because if two threads attempt to clone `Rc`s that point to the same reference-counted value, then they might cause a race condition if they both try to update the reference count at the same time. This produces undefined behavior because `Rc` doesn't use atomic operations. In the context of `spawn`, the closure needs to be passed in *by value* from the thread where it is spawned to the new thread, which means its return value will be passed from the new thread to the thread where it is `join`ed, so it has to implement `Send`.  

>**New to Rust?** TODO! Discuss closures and lifetimes.

Now, let's try to create a new thread. In the example below, we will use `println!` statements to show the activities of two threads running concurrently. Try clicking the play button to see what the program outputs.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("This is {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("This is {} from the main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
You may have noticed that the child thread never got past `i` = 4. This is because it was dropped after the main thread ended. 

`thread::sleep` is a method that forces a thread to stop its execution for a short duration. This allows another thread to run during that time. The threads will probably take turns, but this behavior is not guaranteed -- it is platform-dependent.

Now, let's try a more advanced example, where we use the `JoinHandle` struct returned by `thread::spawn`. We can use `JoinHandle`'s `join` method in order to make a thread wait for other threads to finish. This could solve our problem of not getting past `i` = 4 in the previous example!

```rust 
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("This is {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("This is {} from the main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

The `join` call at the end of the program **blocks** the thread that is currently running until the thread represented by the handle (in this case, the spawned child thread) terminates. When a thread is **blocked**, it is prevented from performing work or exiting. Now, try clicking the play button to see if the output of this example is any different than that of the previous example.

We've covered the basics of `thread::spawn` and `JoinHandle`, so now it's your turn. Try out the exercises below:

* Try moving the `handle.join()` call from the previous example to a different location in the program. Did any of the `println!` statements change? If so, why do you think they did? 

### 12.3.5 &nbsp; Sharing Data Among Threads
The `move` closure can be used alongside `thread::spawn` to allow you to use data from one thread in another. 