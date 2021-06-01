# Chapter 10: System Level I/O
What is I/O? *Computer Systems: A Programmer's Perspective* offers the following definition: 
> **Input/output (I/O)** is the process of copying data between main memory and  external devices such as disk drives, terminals, and networks. An input operation copies data from an I/O device to main memory, and an output operation copies data from memory to a device. 

In this chapter, we discuss how to perform system-level I/O using system-level functions provided by the kernel and higher-level functions provided by programming languages. Specifically, we examine the Unix I/O API and Rust's standard library and compare how to use them for input and output operations. 

Rust programs can invoke Unix I/O system calls directly by using the [`syscall`](https://docs.rs/syscall/0.2.1/syscall/) crate. However, it is usually unnecessary to use these syscalls because Rust provides wrapper functions for most of them. We refer to system calls and their corresponding wrapper functions interchangeably as **system-level functions**.

On Linux systems, we use the system-level **Unix I/O** functions provided by the kernel. Programming languages build on top of these functions to offer higher-level, abstracted facilities for performing I/O. While it is usually sufficient enough to use the higher-level I/O functions provided by programming languages, there are some cases where we may need to use the kernel's I/O functions directly. 

This chapter introduces you to the general concepts of Unix I/O and standard I/O and teaches you how to perform I/O in your own programs. While this chapter serves as a general introduction to I/O, it also lays a solid foundation for other systems concepts. As *Computer Systems: A Programmer's Perspective* states:
> I/O is integral to the operation of a system, and because of this, we often encounter circular dependencies between I/O and other systems ideas. 

We do not discuss the Windows I/O model and the I/O functionalities provided by the Microsoft run-time library, but Rust provides a [Windows-specific I/O crate](https://doc.rust-lang.org/std/os/windows/io/index.html), which you are free to explore on your own!  