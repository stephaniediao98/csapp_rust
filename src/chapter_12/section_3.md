# 12.3 &nbsp; Concurrent Programming with Threads
Thus far, we have looked at two approaches to writing concurrent programs: using processes and using I/O multiplexing. In this section, we discuss a third approach that is a hybrid of these two: threads. We also discuss how to use Rust's [`std::thread`](https://doc.rust-lang.org/std/thread/) crate to perform concurrency. 

## 12.3.1 &nbsp; Threads
A **thread** is a logical flow that runs within the context of a process. Most modern operating systems run multiple threads concurrently in a single process. 

Just like processes, threads are scheduled automatically by the kernel. Furthermore, every thread has its own **thread context**, which includes its **thread ID (TID)**, stack, stack pointer, program counter, general-purpose registers, and condition codes. All threads that run within a process share the process's virtual address space, which includes its code, data, heap, shared libraries, and open files.

## 12.3.2 &nbsp; The Threading Model
Each process begins its life as a single thread called the **main thread**. At some point in the process, the main thread creates a **peer thread**, and then the two threads run concurrently. Eventually, control is passed to the peer thread via a context switch. This can occur because the main thread has called a slow system call, such as `read` or `sleep` or because it is interrupted by the operating system's interval timer. The peer thread then gets to execute for some time before control is passed back to the main thread. This cycle continues while the threads are valid. 

Threads are not organized in a rigid parent-child hierarchy like processes. Threads that are associated with a process form a **pool** of peers. This pool does not consider which threads were created by which other threads. Thus, the only thing really distinguishing the main thread from other threads is that it is the first thread run in the process. Because of this structure, a thread can kill any of its peers, wait for any of its peers to terminate, and read or write shared data at the same time as another peer. 

An executing Rust program consists of a collection of native OS threads. Threads can communicate via [channels](https://doc.rust-lang.org/std/sync/mpsc/index.html), which are Rust's message-passing types, or via other forms of thread synchronization and shared-memory data structures. In Rust programs, fatal errors cause **thread panic**, during which a thread will unwind the stack, running destructors and freeing any owned resources. When the main thread of a Rust program terminates, the entire program is terminated, even if other threads are still running. 

## 12.3.3 &nbsp; How Threads are Represented in Rust
Threads are represented by the [`Thread`](https://doc.rust-lang.org/std/thread/#the-thread-type) type. There are two ways to obtain a `Thread` type: 
1. By spawning a new thread.
2. By requesting the current thread. 

In the remainder of this section, we will discuss the first way to get a `Thread`, but before we get to that, we must first discuss the `std::thread::Thread` struct.

`std::thread::Thread` is a handle to a thread. There is usually no need to create this struct yourself because you can get a `Thread` using one of the two ways listed above. The `Thread` struct is defined as follows: 

```rust, ignore
pub struct Thread {
    inner: Arc<Inner>,
}
```
An `Arc` is a thread-safe reference-counting pointer. "Arc" stands for "Atomically Reference Counted". `Arc<T>`s provide shared ownership of a value of type `T` that is allocated on the heap. Calling [`clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html#tymethod.clone) on `Arc` creates a new `Arc` instance that points to the same allocation on the heap as the source `Arc` and also increases the reference count. When the last `Arc` pointer to an allocation is destroyed, the value stored in that allocation is also destroyed. We also refer to these value as **inner values**.

`Thread` implements methods, such as [`id`](https://doc.rust-lang.org/std/thread/struct.Thread.html#method.id) and [`name`](https://doc.rust-lang.org/std/thread/struct.Thread.html#method.name). The `std::thread` module also contains additional functions that can be performed on threads, such as [`spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html), [`sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html), and [`current`](https://doc.rust-lang.org/std/thread/fn.current.html).

## 12.3.4 &nbsp; Creating Threads
We use the `thread::spawn` function for creating new threads. 

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

>**New to Rust?** The `'static` lifetime is one of Rust's few reserved lifetime names. It is often used in two situations: 
>1. *As a reference with a `'static` lifetime:* In this case, the data pointed to by the reference lives for the entire lifetime of the running program. This case only applies to constants with the `static` declaration and `string` literals which have the type `&'static str`.
>2. *As a trait bound:*: When used as a trait bound, `'static` indicates that the type doesn't contain any non-static references. This means that the receiver of the type can hold onto it for as long as it wants without worrying that it will become invalid until they `drop` it. 

>[**Closures**](https://doc.rust-lang.org/book/ch13-01-closures.html) are anonymous functions that capture their enclosing environment. They can be saved in variables or passed as arguments to other functions.

Now, let's try to create a new thread. In the example below, we will use `println!` statements to show the activities of two threads running concurrently. Try clicking the play button to see what the program outputs.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("This is loop iteration {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("This is loop iteration {} from the main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```
You may have noticed that the child thread never got past `i` = 4. This is because it was dropped after the main thread ended. 

`thread::sleep` is a method that forces a thread to stop its execution for a short duration. This allows another thread to run during that time. The threads will probably take turns, but this behavior is not guaranteed -- it is platform-dependent.

### 12.3.5 &nbsp; Joining Threads

Now, let's try a more advanced example, where we use the `JoinHandle` struct returned by `thread::spawn`. We can use `JoinHandle`'s `join` method in order to make a thread wait for other threads to finish. This could solve our problem of not getting past `i` = 4 in the previous example!

```rust, editable 
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("This is loop iteration {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("This is loop iteration {} from the main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

The `join` call at the end of the program **blocks** the thread that is currently running until the thread represented by the handle (in this case, the spawned child thread) terminates. When a thread is **blocked**, it is prevented from performing work or exiting. Now, try clicking the play button to see if the output of this example is any different than that of the previous example.

We've covered the basics of `thread::spawn` and `JoinHandle`, so now it's your turn. Try out the exercise below:

* Try moving the `handle.join()` call from the previous example to a different location in the program. You can directly edit the code inside the code block then hit the play button to run your code. Did any of the `println!` statements change? If so, why do you think they did? 

### 12.3.6 &nbsp; Blocking and Unblocking Threads
The `std::thread` module provides functions for blocking threads, namely the [`std::thread::park`](https://doc.rust-lang.org/std/thread/fn.park.html) function.

Each `Thread` handle is associated with a token. By default, the token is initially not present. The `park` function blocks a thread unless or until the thread's token is made available. This can be done using the [`std::thread::unpark`](https://doc.rust-lang.org/std/thread/struct.Thread.html#method.unpark) function. The `unpark` function does the opposite of `park`; it atomically makes the token available if it wasn't already. If you are familiar with concurrency in other languages, this may sound a lot like a spinlock to you. `Thread`s in Rust do act very similarly to spinlocks, where they can be unlocked and locked using `unpark` and `park`, respectively. 

The Rust documentation provides two main reasons for implementing `Thread`s in this way:
>It avoids the need to allocate mutexes and condvars when building new synchronization primitives; the threads already provide basic blocking/signaling.
>It can be implemented very efficiently on many platforms.

The following example shows how to use `park` and `unpark` to block and unblock a thread. In the following example, we use the analogy of a `stop_sign` to determine when a car should park or unpark. If `stop_sign` is true, then the car has to `park` until the `stop_sign` is no longer true. Note that we use [`atomic memory Orderings`](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html) in this code to store and load the value of `stop_sign` and `stop_sign2`, which is just a copy of `stop_sign`. We must use atomics in this example because they run completely independent of any other processes, so they help prevent deadlocks and race conditions. Hit the play button to run the example.
```rust
use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;

fn main() {
    let stop_sign = Arc::new(AtomicBool::new(true));
    let stop_sign2 = Arc::clone(&stop_sign);

    let parked_thread = thread::spawn(move || { 
        // The thread spins while at a stop_sign
        while stop_sign2.load(Ordering::Acquire) {
            println!("Parking thread");
            thread::park();
            println!("Received signal to unpark thread");
        }
    });

    thread::sleep(Duration::from_millis(5));
    
    // This sends the signal to unpark the thread
    stop_sign.store(false, Ordering::Release);

    println!("Unparking thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}
```

Eureka, it works! Also, if you're confused about `move` in the above example, don't worry. We cover that in the next section.

### 12.3.7 &nbsp; Terminating Threads
Threads are automatically terminated when the main thread of the process terminates. They also terminate if they encounter a fatal logic error, which causes a **thread panic**. When a thread panics, it unwinds the stack, runs destructors, and frees any owned resources. 