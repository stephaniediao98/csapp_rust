# 10.7 &nbsp; Reading Directory Contents
In this section, we examine how to read directory contents with the [`read_dir`](https://doc.rust-lang.org/std/fs/fn.read_dir.html) function. Reading directory contents is very similar to reading regular files. 

## 10.7.1 &nbsp; The Read_Dir Function
The `read_dir` function is defined as follows: 
```rust, ignore
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir>
```
If the operation is successful, then `Ok(ReadDir)` is returned. Otherwise, an `Err` value is returned. Errors can occur for many reasons, but these are some common ones:

* The provided path doesn’t exist.
* The process lacks the adequate permissions to view the contents.
* The path points refers to a non-directory file.

## 10.7.2 &nbsp; The ReadDir Struct
[`ReadDir`](https://doc.rust-lang.org/std/fs/struct.ReadDir.html) is an iterator over the entries in a directory.

>**New to Rust?** An Iterator is a type that implements the [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait. You can use iterators to easily iterate over collections, such as arrays, vectors, and hashmaps.

The following example shows how to use the `read_dir` function and the `ReadDir` struct to iterate over the entries in a directory, `example_directory`. Assume that `example_directory` includes three files: `file1.txt`, `file2,txt`, and `file3.txt`.

```rust, ignore
use std::io;
use std::fs::{self, DirEntry, read_dir};
use std::path::Path;

fn main() -> io::Result<()> {
    create_dir()
    if let Ok(dir_iter) = read_dir("example_directory") {
        for dir_entry in dir_iter {
            if let Ok(dir_entry) = dir_entry {
                println!("{:?}", dir_entry.file_name());
            }
        }
    }
    Ok(())
}
```
Running the program produces the output: 
```
cargo run
   Compiling section_7 v0.1.0 (/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_7)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_7/target/debug/section_7`
"file2.txt"
"file3.txt"
"file1.txt"
```
You may notice that the files are not printed in order. This is because the order in which `read_dir` returns entries is not guaranteed. You can order the entries by sorting them as follows: 
```rust, ignore
use std::io;
use std::fs::read_dir;

fn main() -> io::Result<()> {
    let mut dir_entries = read_dir("example_directory")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    dir_entries.sort();
    for entry in dir_entries {
        println!("{:?}", entry.file_name().unwrap());
    }
    
    Ok(())
}
```
Running the program produces the output showing the sorted entries:
```
cargo run
   Compiling section_7 v0.1.0 (/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_7)
    Finished dev [unoptimized + debuginfo] target(s) in 0.96s
     Running `/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_7/target/debug/section_7`
"file1.txt"
"file2.txt"
"file3.txt"
```

Run this code to see what entries are in the current directory!
```rust
use std::io;
use std::fs::read_dir;

fn main() -> io::Result<()> {
    if let Ok(dir_iter) = read_dir(".") {
        for dir_entry in dir_iter {
            if let Ok(dir_entry) = dir_entry {
                println!("{:?}", dir_entry.file_name());
            }
        }
    }

    Ok(())
}
```
