# Chapter 8: Exceptional Control Flow

## Exceptions
### Exception Handling
### Classes of Exceptions
#### Interrupts
#### Traps and System Calls
#### Faults
#### Aborts
### Exceptions in Linux/x86-64 Systems
#### Linux/x86-64 Faults and Aborts
#### Linux/x86-64 System Calls
Linux provides hundreds of system calls that application programs use when they want to request services from the kernel, such as reading a file, writing a file, and creating a new process. Some of the popular Unix system calls are below:

| Number      | Name        | Description                          |
| ----------- | ----------- | ------------------------------------ |
| 0           | `read`      | Read file                            |
| 1           | `write`     | Write file                           |
| 2           | `open`      | Open file                            |
| 3           | `close`     | Close file                           |
| 4           | `stat`      | Get info about file                  |
| 9           | `mmap`      | Map memory page to file              |
| 12          | `brk`       | Reset the top of the heap            |
| 32	      | `dup2`	    | Copy file descriptor                 |
| 33          |	`pause`     | Suspend process until signal arrives |
| 37          |	`alarm`	    | Schedule delivery of alarm signal    |
| 39	      | `getpid`	| Get process ID                       |
| 57	      | `fork`      | Create process                       |
| 59	      | `execve`    | Execute a program                    |
| 60	      | `_exit`     | Terminate process                    |
| 61	      | `wait4`     | Wait for a process to terminate      |
| 62          |	`kill`	    | Send signal to a process             |

Notice that each system call has a unique integer number: this number corresponds to an *offset* in a jump table in the kernel so that the processor can quickly and easily jump to the code that carries out each system call. (This jump table is not the same as the exception table.)

Executing system calls in Rust is generally done via the standard library and crates. Rust's I/O APIs provide abstractions over top of system calls that give access to files, processes, networking, and more. Crates can fill in the gaps for system features that Rust does not (or does not yet) have API bindings for.

But how do crates crate new system calls if they, too, are written entirely in Rust? The answer lies in a particularly low-level (and dangerous!) macro: `asm!`.

System calls in Rust must be executed wholly in assembly. The Rust implementation for mixing assembly with Rust code remains unstable and generally inadvisable for production use. As an example of how this might look is as follows:

```rust, ignore
#![feature(asm)]

fn main() {
  unsafe {
    let uid = get_user_id();

    println!("User ID: {}", uid);
  }
}

unsafe fn get_user_id() -> u64 {
  let answer: u64;
  asm!(
    "mov rax, 0x6b",
    "syscall",
    "mov {answer}, rax",
    answer = out(reg) answer,
    lateout("rax") _,
    lateout("r11") _,
    lateout("rcx") _
  );
  answer
}
```

Note that this code (at the time of writing) requires a *nightly* Rust compiler in order to compile. One can be obtained easily using the command `rustup default nightly`.

The system call in Linux for getting the user ID of the user executing the process is `geteuid`, with system call number `0x6b`. To make this system call, we move `0x6b` into `%rax`, execute the `syscall` instruction, and then allow the response from the kernel (placed in `%rax`) to be moved into our variable `answer` in Rust. Any more detail on this process in Rust is outside the scope of this book.

System calls are provided on x86-64 systems via a trapping instruction called `syscall`. It is quite interesting to study how programs can use this instruction to invoke Linux system calls directly. All arguments to Linux system calls are passed through general-purpose registers rather than the stack. By convention, register `%rax` contains the system call number, with up to six arguments in the `%rdi`, `%rsi`, `%rdx`, `%r10`, `%r8`, and `%r9`. The first argument is in `%rdi`, the second in `%rsi`, and so on. On return from the system call, registers `%rcx` and `%r11` are destroyed, and `%rax` contains the return value. A negative return value between -4095 and -1 indicates an error corresponding to the negative `errno`.

## Processes
### Logical Control Flow
### Concurrent Flows
### Private Address Space
### User and Kernel Modes
### Context Switches

## System Call Error Handling 
Thankfully, since Rust handles our system calls internally and translates their results to the Rust `Result` type, there is no special error-handling required for system calls that might be required in other languages. When a function is called that internally requires a system call (for example, reading a file), Rust will handle the response from the kernel and transform it in to a `Result` type accordingly.

## Process Control
Unix provides a number of system calls for manipulating processes. Many of these system calls have abstractions in Rust that permit them to be used in Rust programs. This section describes the most important ones and gives examples for their use.

### Obtaining Process IDs
In Unix, each process has a non-zero 32-bit *process ID (PID)*. The function in Rust that can get this value is `std::process::id`, which takes no arguments and returns a `u32`.

```rust, ignore
pub fn id() -> u32
```
Since processes can in turn spawn other processes, there is also a notion of a process having a *parent* within the kernel. All processes can trace their lineage back to the first process that the kernel spawned while the system was booting: `init`. Though Linux exposes a system call for finding the PID of the *parent* of the currently-running process, no such binding exists in Rust. Very similarly to how raw system calls were made in the **Linux/x86-64 System Calls** section, we can still find a way to make this system call using `unsafe` Rust:

```rust, ignore
#![feature(asm)]

fn main() {
  unsafe {
    let uid = get_parent_process_id();

    println!("Parent Process ID: {}", uid);
  }
}

unsafe fn get_parent_process_id() -> u64 {
  let answer: u64;
  asm!(
    "mov rax, 0x6e",
    "syscall",
    "mov {answer}, rax",
    answer = out(reg) answer,
    lateout("rax") _,
    lateout("r11") _,
    lateout("rcx") _
  );
  answer
}
```

### Creating and Terminating Processes 
From a programmer's perspective, we can think of a process as being in one of three states:
> *Running*. The process is either executing on the CPU or waiting to be executed and will eventually be scheduled by the kernel. *Stopped*. The execution of the process is *suspended* and will not be scheduled. A process stops as a result of receiving a `SIGSTOP`, `SIGTSTP`, `SIGTTIN`, or `SIGTTOU` signal, and it remains stopped until it receives a `SIGCONT` signal, at which point it becomse running again. (A *signal* is a form of software interrupt that will be described in detail soon.) *Terminated*. The process is stopped permanently. A process becomes terminated for one of three reasons: (1) receiving a signal whose default action is to terminate the process, (2) returning from the main routine, or (3) calling the `std::process::exit` function.

The third method of terminating a process relies on a function provided by the `std::process` module:
```rust, ignore
pub fn exit(code: i32) -> !
```

Recall that the `!` return type indicates that a function does not return control to the calling function.

New processes can be spawned using the `std::process::Command` module. Assume that there exists some small binary file called `spawn` that exists in the working directory of a Rust program. This binary file prints "Hello world!" to standard output when it runs. The following code would work to execute that binary and print the output that it provides:

```rust, ignore
use std::process::Command;

fn main() {
  println!("Spawning child process...");

  let out = Command::new("./spawn").output().expect("Failed to spawn new process");
  let out_string = String::from_utf8(out.stdout).unwrap_or_else(|_| {
    String::from("Standard output from spawned process contained unsupported characters")
  });

  print!("{}", out_string);
}
```
Internally, Rust's spawning of this process makes heavy use of the system calls `fork` and `execvp`. `fork` exists to clone the current process. This new process is exactly the same in every way *except* that the process ID is different. Moreover, `fork`, by its nature, returns *two* times — once in the original process and once in the new child process. It returns with different values for each, though: the parent receives the child's PID and the child receives `0`. In this way, subsequent code can use the returned value from `fork` to decide on different tasks to perform.

In the case of Rust spawning a new process, the child process that is spawned once `fork` has been called checks for the return value from `fork`. In the case that it is `0` and the running process is the *child* process, Rust then proceeds to execute another system call: `execvp`. This, as would be necessary to execute a file, is a request to the kernel to execute the binary code contained in the file passed to the system call.

One of the important notes about the `fork` system call is that it duplicates the memory space of the calling process for the child. This means that changes made in the child process will *not* be reflected in the parent, since their memory space is (at the instant of process cloning) identical, but not the same space in memory.

### Reaping Child Processes
When a process terminates for any reason, the kernel does not remove it from the system immediately. Instead, the process is kept around in a terminated state until it is *reaped* by its parent. When the parent reaps the terminated child, the kernel passes the child's exit status to the parent and then discards teh terminated process, at which point it ceases to exist. A terminated process that has not yet been reaped is called a *zombie*.

When a parent process terminates, the kernel arranges for the `init` process to become the adopted parent of any orphaned children. If a parent process terminates without reaping its zombie children, then the kernel arranges for the `init` process to reap them. Long-running programs such as shells or servers, however, should always reap their zombie children. Even though zombies are not running, they still consume system memory resources.

A process waits for its children to terminate or stop by calling the `waitpid` function. In Rust, this system call is used internally when `Command` spawns a new process. `waitpid` provides ways for the calling process to examine many of its child processes at once, and encodes information in its return value about the exit codes from those processes so that the parent can take action based on whether the child process succeeded or failed.

### Putting Processes to Sleep
Rust has a function called `std::thread::sleep` which enables the thread to request to be suspended for a certain amount of time.

```rust, ignore
pub fn sleep(dur: Duration)
```
It is guaranteed that the program will sleep for no *less* than the amount of time given, though slight variations in the operating system's scheduler will usually cause the program to sleep for a small amount longer than requested. Internally, this uses the `nanosleep` system call, causing the process to be unscheduled by the kernel *unless* a signal needs to be handled by the program. In this case, the `nanosleep` call may need to be made by Rust many times to ensure that the appropriate amount of time is slept through, since upon completing the handling of the signal, the program resumes execution. Luckily, Rust handles these intricacies internally.

## Signals
...

## Nonlocal Jumps

## Tools for Manipulating Processes

## Summary