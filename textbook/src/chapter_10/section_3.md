# 10.3 &nbsp; Opening and Closing Files
In this section, we discuss how to open and close files using Rust's [standard library](https://doc.rust-lang.org/std/). The Rust standard library contains the [`std::io`](https://doc.rust-lang.org/std/io/index.html) and [`std::fs`](https://doc.rust-lang.org/std/fs/index.html) modules for performing I/O and manipulating the filesystem, respectively.

## 10.3.1 &nbsp; Open
The `File` struct has an `open` method implemented for it: 
```rust, ignore
pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
        OpenOptions::new().read(true).open(path.as_ref())
    }
```

>**New to Rust?**
> [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) is a struct from the [`std::path`](https://doc.rust-lang.org/std/path/index.html) module that represents a slice of a path. It is similar to the `str` type. `Path` is really just a wrapper around `OsStr`, which is why they work directly on string according to the local platform's (in this case Unix) path syntax. This is an unsized type, which means that it must always be used behind a pointer like `&` or `Box`. `AsRef` is used to perform cheap reference-to-reference conversions. Thus, the `<P: AsRef<Path>>` code specifies that the path argument must implement this trait. A path implements this trait if it can be converted to reference to `Path`.

>**Traits** define shared behavior among different types. The Rust compiler interprets traits as some functionality a particular type has and can share with other types. A type has the behavior specified by a trait if that trait is *implemented* for the type.  This basically means that we can call the functions defined for a trait on all types that have, or implement, that trait. 

The `File::open` function takes a path argument and returns an `io::Result<File>`, where [`std::io::Result`](https://doc.rust-lang.org/std/io/type.Result.html) is the specialized `Result` type for I/O operations. If `File::open` is successful, `Ok(File)` is returned. Otherwise, [`io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) is returned. 

```rust, ignore
type Result<T> = Result<T, Error>;
```

>**Why does `std::io` use an alias of `std::result::Result`?**
When a `Result` type is used, it is generally assumed to be the [`std::result::Result`](https://doc.rust-lang.org/std/result/enum.Result.html) type. However, the I/O crate has specified an alias for `Result` that is used for I/O operations. This was done in order to avoid writing out `io::Error` directly.

>**New to Rust?**
The `Result` type is an enum with two variants: `Ok(T)` and `Err(E)`. The `Ok(T)` variant indicates that the operation was successful and returned a value of type `T`, while the `Err(E)` variant indicates that the operation was not successful and returned an error value of type `E`. 
```rust, ignore
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```
Now, let's put this all together and go through an example. Assume we have a file we want to open, `open_me.txt` that is in the same directory as `main.rs`.

```rust
use std::fs::File;

fn main() {
    let result = File::open("open_me.txt");
    println!("The result of the open operation was: {:?}", result);
}
```

Note that if you hit the play button to run the code, you will see a "No such file or directory" error. This happens because the textbook's browser has no idea where to find the file. Running the program in your local directory returns: 
```
cargo run
   Compiling chapter_10_code v0.1.0 (../chapter_10_code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `../chapter_10_code/target/debug/chapter_10_code`
The result of the open operation was: Ok(File { fd: 3, path: "/../chapter_10_code/src/open_me.txt", read: true, write: false })
```

We can see that we were able to successfully open `open_me.txt`. The `File`'s file descriptor, path, and read and write permissions are returned. 

## 10.3.2 &nbsp; OpenOptions
You may have noticed an [`OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html) struct being initialized in the function body of `File::open`. This struct encapsulates the options and files that can be used to configure how a file is opened. It also has an `open` method implemented for it. In fact, if you look back at the `File::open` code, you can see that `File::open` opens a file using `OpenOptions::open`. This is because `File::open` is actually an alias for `OpenOptions::open`. 

```rust, ignore
pub fn open<P: AsRef<Path>>(&self, path: P) -> io::Result<File> {
        self._open(path.as_ref())
    }
```

`OpenOptions` has the same function definition as `File::open`, and the two can be used interchangeable. However, it is useful to use `OpenOptions` when you want to specify permissions or flags options for the file. This can be done with the `OpenOptions::new` method, which creates a blank new set of file options ready for configuration. All options are initially set to `false`. The permissons and flags options you can set include: 
* read - Read permissions. If `true`, then the file should be read-able.
* write - Write permissions. If `true`, then the file should be write-able.
* append - Open the file in *append mode*. Before each `write` operation, position the file offset at the end of the file, as if with `seek`. This prevents overwriting previous content. 
* truncate - If the file exists, then truncate it to length 0. The file must be opened with write access.
* create - Create a new file or open it if already exists.
* create_new - Creates a new file or fails if it already exists.
* custom_flags - Sets custom flags bits. (System-specific)
* mode - Sets the mode bits that a new file will be created with. The operating system masks out bits with the system's `umask` to produce the file's final permissons. Each process includes a `umask` in its context that is set by calling the `umask` function. When a process calls the `OpenOptions::open` function with some `mode` argument and creates a new file, the access permission bits of the new file are set to `mode & ~umask`. If no `mode` is set, then the default value `0o666` is used. (System-specific)

Just like the `std::fs::File` struct, `std::fs::OpenOptions` derives itself from [`std::sys::unix::fs::OpenOptions`](https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/std/sys/unix/fs/struct.OpenOptions.html):

```rust, ignore
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    custom_flags: i32,
    mode: mode_t,
}
```
The `custom_flags` and `mode` fields are derived from the [`libc`](https://docs.rs/libc/0.2.95/libc/) crate. The symbolic names and permissions for these bits are shown in the table below:

<br>

| Mask    | Permissions                                       |
| ------- | ------------------------------------------------- |
| S_IRUSR | User (owner) can read the file                    |
| S_IWUSR | User (owner) can write the file                   |
| S_IXUSR | User (owner) can execute the file                 |
| S_IRGRP | Members of the owner's group can read the file    |
| S_IWGRP | Members of the owner's group can write the file   |
| S_IXGRP | Members of the owner's group can execute the file |
| S_IROTH | Others (anyone) can read the file                 |
| S_IWOTH | Others (anyone) can write the file                |
| S_IXOTH | Others (anyone) can execute the file              |

<br>
<center><b> Table 10.1: Access permission bits </b></center>
<br>

Now, let's try the example from 10.3.1 with `OpenOptions::open` instead of `File::open`. This code is in the `chapter_10_code/src/section_3/open_options.rs` file.
```rust, ignore
use std::fs::OpenOptions;

fn main() {
    let result = OpenOptions::new().read(true).open("open_me.txt");
    println!("The result of the open operation was: {:?}", result);
}
```

Running the program yields the following result: 
```
cargo run
   Compiling chapter_10_code v0.1.0 (../chapter_10_code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `../chapter_10_code/target/debug/chapter_10_code`
The result of the open operation was: Ok(File { fd: 3, path: "../chapter_10_code/src/open_me.txt", read: true, write: false })
```
Great! We can see that both `File::open` and `OpenOptions::open` returned the same result. Now, let's consider an example where you want to open a file with both read and write permissions set.

```rust, ignore
use std::fs::OpenOptions;

fn main() {
    let result = OpenOptions::new().read(true).write(true).open("open_me.txt");
    println!("The result of the open operation was: {:?}", result);
}
```
```
cargo run
   Compiling chapter_10_code v0.1.0 (../chapter_10_code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `../chapter_10_code/target/debug/chapter_10_code`
The result of the open operation was: Ok(File { fd: 3, path: "../chapter_10_code/src/open_me.txt", read: true, write: true })
```
We can see that both `read` and `write` are set to `true`, so it worked! Here are a couple exercises for you to try: (You can assume that the file, `file.txt` is in the same directory as `main.rs`.)

* How would you open a file with `create` set? 
* How would you open a file with `truncate` set? 
* How would you open a file in a mode where anyone can read the file? 

The solutions to these exercises are below. Click the arrow button to unhide them!
<details>
  <summary>Exercise Solutions</summary> 

  ```rust, ignore
    // You will need to add `libc = "0.2"` to `[dependencies]` in your Cargo.toml file.
    extern crate libc; 
    use std::fs::OpenOptions;
    use std::os::unix::fs::OpenOptionsExt;

    fn main() {
        // Opens the file with `create` set
        let _create_file = OpenOptions::new().read(true).write(true).create(true).open("file.txt");

        // Opens the file with `truncate` set 
        let _trunc_file = OpenOptions::new().write(true).truncate(true).open("file.txt");

        // Opens the file such that anyone can read it
        let mut options = OpenOptions::new();
        options.write(true);
        if cfg!(unix) {
            options.mode(libc::S_IROTH.into());
        }
        let _mode_file = options.open("file.txt");
    }
 ```
</details>

## 10.3.3 &nbsp; Close
Files are automatically closed once they go out of scope.

```rust, ignore
fn main() {
    let file = File::open("file.txt"); // file is now in scope
    // do things with the file
} // file goes out of scope and is automatically closed
```
Errors that are detected upon closing are ignored because `FileDesc` implements the [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait, which acts as a destructor on a value once it is no longer needed. In order to handle errors, you must use the method [`sync_all`](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all). `Sync_all` attempts to sync all OS-internal metadata to the filesystem. Dropping a file will ignore errors in synchronizing the in-memory data. Here is an example of how you might use `sync_all`:

```rust, ignore
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("file.txt")?;
    file.write_all(b"Hello, world!")?;
    // do some stuff with the file
    
    file.sync_all()?;
    Ok(())
}
```
>**New to Rust?**
The `()` type is known as the **unit** type in Rust. The unit type has one value, `()`, and it is used when there is no other meaningful value that can be returned. Functions that do not explictly declare a return type automatically return the unit type. Therefore, the following two function definitions have equivalent return types:
> ``` rust, ignore
> fn func1() -> () {}
> fn func2() {}
> ```


