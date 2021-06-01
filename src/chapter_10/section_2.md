# 10.2 &nbsp; Files 
## 10.2.1 &nbsp; Linux File Types
The Linux system categorizes files into different **types**, where a file's type indicates its role in the system. Linux file types include:  
* *Regular files:* These contain arbitrary data. There are two subtypes: *text files* and *binary files*. Text files are files that contain only ASCII or Unicode characters, and binary files are all other regular files. Applications usually distinguish between these two, but the kernel doesn't.
* *Directories:* These are files that maintain the mappings between filenames and files, which can be other directories. You can think of them as folders in your filesystem. They contain an array of *links*, where each link is the mapping between a filename and a file. Each directory contains at least two entries: 
    1) `.` (dot) - a link to the directory itself 
    2) `..` (dot-dot) - a link to the *parent directory* in the directory hierarchy
* *Sockets:* These types of files are used in network programming to communicate with another process across a network.

There are other types of files specified by the Linux system, but those are beyond our scope for now. 

## 10.2.2 &nbsp; The Linux Filesystem
The Linux filesystem is organized as a **single directory hierarchy** that is anchored by a **root directory** named `/` (slash). This structure is similar to that of a tree. Every file in the filesystem is a child of the root, either directly or indirectly.  

Pathnames identify locations in the directory hierarchy. They are formatted strings that follow the pattern `/filename1/filename2/.../filename3`, where the first slash `/` is optional. There are two forms of pathnames: 
1. *Absolute pathnames:* These pathnames start with a slash, and they denote a path from the root node. 
2. *Relative pathnames:* These pathnames start with a filename, and they denote a path from the current working directory, not the root. 

## 10.2.3 &nbsp; A Short Aside on Processes
We mentioned earlier that I/O is closely related to other systems ideas. One of these ideas is a process. Processes and I/O often work together, as I/O plays a central role in process creation and execution, while process creation plays a central role in how files are shared betwen processes. 

A **process** is an instance of a program being executed. One component of a process is its **context**, which saves information about its current state. One piece of information that is included in a process's context is its current location, which is also known as its **current working directory**. You can change your shell's current working directory using the `cd` command.

## 10.2.4 &nbsp; How Files are Represented in Rust
In the upcoming sections, we will show you how to use Rust's standard library to perform I/O. Most of these I/O functions revolve around the [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) struct in Rust's standard library. However, I/O functions can also be performed on other types, such as [`TcpStream`](https://doc.rust-lang.org/std/net/struct.TcpStream.html) and [`Vec<T>`](https://doc.rust-lang.org/std/vec/struct.Vec.html).

The `File` struct represents a reference to an open file on the filesystem. 

```rust, ignore
pub struct File {
    inner: fs_imp::File,
}
```
`File` contains one member: `inner`, which is of the type `fs_imp::File`, where `fs_imp` is an identifier for the [`sys::fs`] crate. The `sys::fs` crate is part of the internal docs and is not part of Rust's public API. However, we will provide a brief overview of its `File` struct so that you can have a general idea of what's going on under the hood. Furthermore, since we're focusing on Unix in this textbook, we specifically want to look at [`std::sys::unix::fs::File`](https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/std/sys/unix/fs/index.html). 

```rust, ignore
pub struct File(FileDesc);
```
[`FileDesc`](https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/std/sys/unix/fd/struct.FileDesc.html) comes from the [`std::sys::unix::fd`](https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/std/sys/unix/fd/index.html) module and is a `c_int`, which is defined in [`libc::c_int`](https://docs.rs/libc/0.2.43/libc/type.c_int.html) as an `i32`. The `std::sys::unix::fd` is a nightly-only experimental API, so you will need to have `nightly` installed in order to directly run code from the `std::sys::unix` crate. 

In short, `File` is a generic type that contains one member, `inner`, which implements the system-specific file representation for the appropriate platform. On Unix systems, `inner` is a `std::sys::unix::fs::File` type, where the file contians a file descriptor, `FileDesc`.