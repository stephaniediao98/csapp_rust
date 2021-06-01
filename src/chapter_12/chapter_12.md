# Chapter 12: Concurrent Programming
As we learned in Chapter 8, logical control flows are **concurrent** if they overlap in time. This concept of **concurrency** appears at many different levels of a computer system. Examples include hardware exception handlers, processes, and Linux signal handlers. 

So far, we have discussed concurrency in the context of the operating system kernel running multiple application programs at once. However, concurrency is not just limited to the kernel; it can also be used in user application programs. This is called **application-level concurrency**, and it includes accessing slow I/O devices, servicing multiple network clients, parallel computing on multi-core machines, and responding to asyncronous events. Applications that use application-level concurrency are known as **concurrent programs**. 

Programming languages implement threading APIs in a few different ways. One way relies on the operating system's provided threading API. In this model, which we call **1:1**, one operating system thread corresponds to one language thread. Another way is the **M:N** model, where there are `M` programming language-provided threads (also known as **green threads**) per `N` operating system threads. 

Each threading model has its own pros and cons. According to the Rust documentation, the trade-off most important to Rust is runtime support, where **runtime** means code that is included by the language in every binary. Some languages are okay with sacrificing runtime in exchange for more features, but Rust is not becuse it cannot compromise on being able to call into C to maintain performance. Given this, Rust implements the 1:1 model instead of the green-threading M:N model. However, since Rust is a low-level language, you can find crates that implement M:N threading, such as the [`futures::executor`](https://docs.rs/futures/0.3.15/futures/executor/index.html) crate. We will not discuss such crates because they are beyond the scope of this textbook, but you are welcome to explore them on your own. 

On the operating system side, three basic approaches are provided for building concurrent programs:

1. Processes
2. I/O Multiplexing
3. Threads

Handling concurrency safely and efficiently is a challenge due to the potential problems that could arise, like **race conditions**, **deadlocks**, and more. Race conditions occur when multiple threads access the same chunk of data or resources in an inconsistent order. Deadlocks occur when two threads are blocked from continuing because they are waiting on the other thread to finish using a resource the other one has. Another issue that arises is undefined or unreproducable behavior. When we can't figure out why a bug is occurring or can't reproduce a bug, it becomes very difficult to fix the bug. Luckily, one of Rust's prized features is **fearless concurrency**, which allows us to perform concurrency easily, safely, efficiently, and reliably.

In this chapter, we examine how to write concurrent programs using processes, I/O multiplexing, and threads. Our main focus throughout this chapter will be on concurrency via threads because Rust's `std::thread` crate is particularly robust and enables us to perform fearless concurrency. 
