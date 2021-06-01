# Chapter 12: Concurrent Programming
As we learned in Chapter 8, logical control flows are **concurrent** if they overlap in time. This concept of **concurrency** appears at many different levels of a computer system. Examples include hardware exception handlers, processes, and Linux signal handlers. 

So far, we have discussed concurrency in the context of the operating system kernel running multiple application programs at once. However, concurrency is not just limited to the kernel; it can also be used in user application programs. For instance, Linux signal handlers allow applications to respond to asynchronous events, such as the user typing `Ctrl+C` or the program accessing and undefined area of virtual memory. Application-level concurrency is also useful in many other ways, such as: 

* *Accessing slow I/O devices*
* *Interacting with Humans* 
* *Reducing latency by deferring work*
* *Servicing multiple network clients*
* *Computing in parallel on multi-core machines* 

Applications that use application-level concurrency are known as **concurrent programs**. Handling concurrency safely and efficiently is a challenge, but Rust's *fearless concurrency* features makes it much easier. 

Modern operating systems provide three basic approaches for building concurrent programs:

* *Processes*
* *I/O multiplexing*
* *Threads*

In this chapter, we examine these three techniques for writing concurrent programs. In particular, we will focus on building a concurrent echo server.ß
