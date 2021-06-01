# 10.1 &nbsp; Unix I/O
Linux is in the class of Unix-like operating systems, which all behave similarly to the Unix operating system. For example, they all support certain features, such as multiple users, separation between kernel mode and user mode, and a hierarchical file system, which we discuss in 10.2. They also usually share one of the defining features of Unix: "Everything is a file". [[1]](https://www.computerhope.com/jargon/u/unix-like.htm) 

## 10.1.1 &nbsp; Everything is a file
On Linux systems, a **file** is represented as a sequence of *m* bytes:

<center><i>B<sub>0</sub>, B<sub>1</sub>, B<sub>2</sub>,..., B<sub>m-1</sub></i></center>
<br>

All I/O devices (such as networks, disks, terminals, and even the kernel itself) are represented as files, so everything is essentially a file, which is really just a sequence of bytes. This straightforward mapping of I/O devices to files makes it easy for Linux to develop a simple, low-level API around it. Furthermore, since everything is represented with one main communication primitive (the file), the API can ensure that it performs in a uniform and consistern manner. We call this API **Unix I/O**. 

## 10.1.2 &nbsp; Unix I/O Functions
The following list shows the standard ways in which Unix I/O's functions are performed: 

1. *Opening files:* When an application wants to access a file, it asks the kernel to `open` the file. In response, the kernel sends back a **descriptor**, which is a small nonnegative integer that identifies the file while it is open. The application keeps track of the descriptor, while the kernel keeps track of the the open file.
2. *Changing the current file position:* The kernel maintains a file position *k* for each open file, where *k* is the byte offset from the beginning of a file. You can think of the file position like a cursor. An application can change *k* by performing a `seek` operation.
3. *Reading and writing files:* When an application requests to `read` from a file, the contents of the file are copied to memory. A read operation copies *n* bytes from the file, starting at the current file position *k* and then incrementing *k* by *n*. If a read operation is called where *k* is greater than the file's size, then an end-of-file (EOF) condition is triggered and detected by the calling application. The `write` operation works in a similar way, except instead of copying bytes from a file to memory, bytes are copied from memory to a file.  
5. *Closing files:* Once an application is done accessing a file, it asks the kernel to `close` the file. The kernel consequently frees all the data structures that were created when opening the file and returns the file's descriptor to the pool of available descriptors.