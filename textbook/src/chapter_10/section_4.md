# 10.4 &nbsp; Reading and Writing Files
In this section, we discuss the [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) and [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) traits, which provide a general interface for reading and writing input and output. 

## 10.4.1 &nbsp; Read
The `Read` trait reads bytes from a source. A type that implements this trait is called a **reader**. Types such as `File`s, `TcpStream`s, and `Vec<T>`s are readers. The `Read` trait defines multiple read functionalities, with a core [`read`](https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read) method that attempts to pull bytes from a source into a provided buffer. The other read methods build off of this one `read` method and include [`read_to_end`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_end), [`read_to_string`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string), [`read_exact`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact), and [`read_vectored`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_vectored). Readers are only required to implement the `read` method, which gives them the ability to use the other read methods.

Before we get into the different ways we can read files, let's take a closer look at the `read` method that readers are required to implement.

```rust, ignore
fn read(&mut self, buf: &mut [u8]) -> Result<usize>
```
The `read` method copies (at most) *n* bytes from the current file position of the file identified by descriptor *fd* to memory location *buf*. If `read` is successful, then `Ok(usize)` is returned, where `usize` is the number of bytes read. If the number of bytes returned is 0, then either the reader has reached the end of the file or the buffer was initialized with length 0. If `read` is unsuccessful, then it returns an `Err` value. 

In some situations, `read` and `write` copy less bytes than requested by the application. These are known as **short counts**, and they do not result in an error. They may occur for the following reasons: 

* *Encountering EOF on reads:* Sometimes we want to read from a file that has less than *n* bytes from the current file position to the EOF, which causes a short count. For instance, suppose we want to read from a file that contains 20 more bytes from the current file position and that we want to read the file in 50-byte chunks. In this case, the next`read` will return a short count of 20, and the `read` after that will signal that it has reached the end of the file by returning a short count of 0. 
* *Reading text lines from a terminal:* If an open file is associated with a terminal (i.e. a keyboard and display), then the number of bytes that can be read from the file is restricted by a text line in the terminal. Each `read` function copies one text line at a time, returning a short count equal to the number of bytes of the text line. 
* *Reading and writing network sockets:* If an open file is associated with a network socket (which we discuss more in Chapter 11), then internal buffering constraints and long network delays can cause `read` and `write` to return short counts. Short counts can also occur when `read` and `write` are called on a Linux **pipe**, which is an interprocess communication mechanism. Pipes are beyond our scope, but they basically let you use multiple commands such that the ouptut of one is passed as input to the next.
* *Read was interruped by a signal*.

Other than these cases, you will typically not encounter short counts, especially if you are reading from a disk file (except when you reach the end of the file).

Let's try an example where we read from a file, `read_me.txt` that is located in the same directory as `main.rs`. 

```rust, ignore
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("read_me.txt")?;
    let mut buffer = [0; 25];

    // read at most 25 bytes from the file
    let num_bytes = f.read(&mut buffer)?;
    println!("Read {} bytes from the file.", num_bytes);

    Ok(())
}
```

Running the program in your local directory returns: 
```
cargo run
   Compiling section_4_code v0.1.0 (../chapter_10_code/src/section_4_code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `../section_4_code/target/debug/section_4_code`
Read 13 bytes from the file.
```

We can also read the contents of a file into a string. Let's write some text inside `read_me.txt`:
```
hello, world!
```

Now, let's try reading what we just wrote in the file:

```rust, ignore
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("read_me.txt")?;
    let mut string_buffer = String::new();

    f.read_to_string(&mut string_buffer)?;
    println!("The file says: {}", string_buffer);

    Ok(())
}
```

```
cargo run
   Compiling section_4_code v0.1.0 (../chapter_10_code/src/section_4_code)
    Finished dev [unoptimized + debuginfo] target(s) in 1.33s
     Running `../chapter_10_code/src/section_4_code/target/debug/section_4_code`
The file says: hello, world!
```

>**New to Rust?** The [`?`](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html) operator allows for easier error handling. You can apply `?` to functions that return a `Result` value. If the function return the `Ok` variant, then `?` unwraps it and returns the inner value. Otherwise, if it was the `Err` variant, then `?` returns from the function you are currently in. Using `?` in place of `match` statements or other forms of handling `Result` values is much more straightforward and visually clean. 

Here are some exercises for you to try: 

* Open a file with the `read` option set to false. Then try reading from it. What happens? 
* Read the contents of `read_me.txt` into a vector. Then print the vector. (Hint: To ensure that all contents in the file are copied to the vector, use `read_to_end`).

The solutions to these exercises are below. Click the arrow button to unhide them!
<details>
  <summary>Exercise Solutions</summary> 

  ```rust, ignore
    use std::io;
    use std::io::prelude::*;
    use std::fs::OpenOptions;

    fn main() -> io::Result<()> {
        let mut unreadable_file = OpenOptions::new().read(false).create(true).open("file.txt")?;
        let mut string_buffer = String::new();
        // This returns: Error: Os { code: 22, kind: InvalidInput, message: "Invalid argument" }
        unreadable_file.read_to_string(&mut string_buffer)?;

        let mut f = OpenOptions::new().read(true).open("read_me.txt")?;
        let mut vec_buffer = Vec::new();
        f.read_to_end(&mut vec_buffer)?;
        // This returns: The file says: [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]
        println!("The file says: {:?}", vec_buffer);

        Ok(())
    }
 ```
</details>

## 10.4.2 &nbsp; Write
The `Write` trait writes data into an object, which is a byte-oriented sink. Types that implement the `Write` trait are also known as **writers**. Writers are required to implement two methods: `write` and `flush`:

* `write` - Writes data into an object and returns the number of bytes that were written.
* `flush` - Ensures that all buffered data has been pushed out to the "true sink".

Other methods of the `Write` trait include [`write_all`](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all), [`write_vectored`](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_vectored), [`write_all_vectored`](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all_vectored), and [`write_fmt`](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_fmt). `File` implements `write`, `write_vectored`, and `flush`. However, we can still use the other `write` methods because they build off the core two methods `write` and `flush`. 

```rust, ignore
fn write(&mut self, buf: &[u8]) -> Result<usize>
```
The `write` method writes at most *n* bytes from a buffer into the writer and returns `Ok(usize)` if the operation is successful, where `usize` is the number of bytes written, or an `Err` value if the operation was unsuccessful. If the number of bytes returned is 0, then either the underlying object is no longer able to accept any more bytes or the provided buffer is empty. Just like `read`, short counts are not considered errors for `write`. Furthermore, if the `write` function is interrupted, an `ErrorKind::Interrupted` error is raised but is non-fatal, so the `write` operation can be tried again if there is nothing else to do. 

```rust, ignore
fn flush(&mut self) -> Result<()>
```

The `flush` method flushes the output stream, ensuring that all buffered contents reach their destination. If a short count is encountered, then this does produce an error, unlike `read` and `write`, even if the short count occurs because it has reached the end of the file. 

The example below shows how to write to a file. We can check if the operation worked by reading the file. 

```rust, ignore
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("write.txt")?;
    buffer.write(b"hello file!")?;
    
    let mut file_contents = String::new();
    let mut f = File::open("write.txt")?;
    f.read_to_string(&mut file_contents)?;
    println!("The file says: {}", file_contents);

    Ok(())
}
```
Running the program outputs: 
```
cargo run
   Compiling section_4_code v0.1.0 (/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_4_code)
    Finished dev [unoptimized + debuginfo] target(s) in 0.80s
     Running `/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_4_code/target/debug/section_4_code`
The file says: hello file!
```
Great! We know it works because we were able to read what we wrote to the file. Now, let's try an example with `flush`.

```rust, ignore
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("flush.txt")?;

    buffer.write_all(b"All these bytes should be written!")?;
    buffer.flush()?;

    let mut file_contents = String::new();
    let mut f = File::open("flush.txt")?;
    f.read_to_string(&mut file_contents)?;
    println!("The file says: {}", file_contents);

    Ok(())
}
```
```
cargo run
   Compiling section_4_code v0.1.0 (/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_4_code)
    Finished dev [unoptimized + debuginfo] target(s) in 1.05s
     Running `/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_4_code/target/debug/section_4_code`
The file says: All these bytes should be written!
```

## 10.4.3 &nbsp; BufReader and BufWriter
[`BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html) and [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html) are two structs included in `std::io` that allow for more efficient reads and writes. The Rust documentation provides the following reason for why `BufReader` is particularly helpful: 

>It can be excessively inefficient to work directly with a `Read` instance. For example, every call to read on `TcpStream` results in a system call. A `BufReader<R>` performs large, infrequent reads on the underlying `Read` and maintains an in-memory buffer of the results.
`BufReader<R>` can improve the speed of programs that make small and repeated read calls to the same file or network socket. It does not help when reading very large amounts at once, or reading just one or a few times. It also provides no advantage when reading from a source that is already in memory, like a `Vec<u8>`.

`BufReader` also provides additional ways of reading files, such as [`read_until`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_until), [`read_line`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line), [`split`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.split), and [`lines`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines). 

`BufWriter` is also much more efficient than `Write`. The Rust documentation provides the following reason for why `BufWriter` is helpful: 

>It can be excessively inefficient to work directly with something that implements `Write`. For example, every call to write on `TcpStream` results in a system call. A `BufWriter<W>` keeps an in-memory buffer of data and writes it to an underlying writer in large, infrequent batches.
`BufWriter<W>` can improve the speed of programs that make small and repeated write calls to the same file or network socket. It does not help when writing very large amounts at once, or writing just one or a few times. It also provides no advantage when writing to a destination that is in memory, like a `Vec<u8>`.
It is critical to call `flush` before `BufWriter<W>` is dropped. Though dropping will attempt to flush the contents of the buffer, any errors that happen in the process of dropping will be ignored. Calling `flush` ensures that the buffer is empty and thus dropping will not even attempt file operations.

Let's go through an example for how to read and write from a file using `BufReader` and `BufWriter`. Specifically, we will try to copy text from one file to another, line by line. First, let's create a file `poem.txt` that contains the poem *When I Am Gone* by Shel Silverstein: 
```
When I am gone what will you do?
Who will write and draw for you?
Someone smarter—someone new?
Someone better—maybe YOU!
```
Let's start by just reading the file, line by line.

```rust, ignore
use std::io::prelude::*;
use std::io::{BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("poem.txt")?;
    let bufreader = BufReader::new(f);

    for line in bufreader.lines() {
        println!("{:?}", line);
    }

    Ok(())
}
```
```
cargo run
   Compiling section_4_code v0.1.0 (/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_4_code)
    Finished dev [unoptimized + debuginfo] target(s) in 1.19s
     Running `/Users/stephaniediao/Desktop/rust/chapter_10_code/src/section_4_code/target/debug/section_4_code`
Ok("When I am gone what will you do?")
Ok("Who will write and draw for you?")
Ok("Someone smarter—someone new?")
Ok("Someone better—maybe YOU!")
```
It works! So now let's try using `BufWriter` to write each line to another file, `poem_copy.txt`.

```rust, ignore
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f1 = File::open("poem.txt")?;
    let f2 = File::create("poem_copy.txt")?;
    let bufreader = BufReader::new(f1);
    let mut bufwriter = BufWriter::new(f2);

    for line in bufreader.lines() {
        bufwriter.write(&line.unwrap().as_bytes())?;
        bufwriter.write(b"\n")?;
    }

    Ok(())
}
```
After running the program, `poem_copy.txt` looks like this: 
```
When I am gone what will you do?
Who will write and draw for you?
Someone smarter—someone new?
Someone better—maybe YOU!
```
This means our program works correctly and has copied the contents of `poem.txt` to `poem2.txt`, line by line. Now, it's your turn! Try out the following exercises:

* Given the text file below, read the contents of the file until you have reached the first ',' delimeter. Double check that your program has stopped at the first ','. How many bytes were read?
* Then, split the text file on each whitespace.

`exercise.txt`:
```
It was the best of times, it was the worst of times, it was the age of wisdom, it was the age of foolishness, it was the epoch of belief, it was the epoch of incredulity, it was the season of Light, it was the season of Darkness, it was the spring of hope, it was the winter of despair, we had everything before us, we had nothing before us, we were all going direct to Heaven, we were all going direct the other way—in short, the period was so far like the present period, that some of its noisiest authorities insisted on its being received, for good or for evil, in the superlative degree of comparison only.
```

The solutions to these exercises are below. Click the arrow button to unhide them!
<details>
  <summary>Exercise Solutions</summary> 

  ```rust, ignore
    use std::io::prelude::*;
    use std::io::{BufReader};
    use std::fs::File;

    fn main() -> std::io::Result<()> {
        let f = File::open("exercise.txt")?;
        let mut bufreader = BufReader::new(f);
        let mut buffer = Vec::new(); 

        // Read from the file until we hit the first , delimeter
        let num_bytes = bufreader.read_until(b',', &mut buffer)?;
        // This outputs: "Number of bytes read: 25"
        println!("Number of bytes read: {}", num_bytes);
        // This outputs: "Text read: It was the best of times,"
        println!("Text read: {}", String::from_utf8(buffer).unwrap_or("Something went wrong.".to_string()));

        let split_iter = bufreader.split(b' ');
        for split in split_iter {
            // This outputs each word in the file
            println!("{:?}", String::from_utf8(split.unwrap()).unwrap_or("Something went wrong.".to_string()));
        }

        Ok(())
    }
 ```
</details>