# 10.6 &nbsp; Reading File Metadata
In this section we discuss how to retrieve information about a file, also known as a file's **metadata**, using the [`metadata`](https://doc.rust-lang.org/std/fs/fn.metadata.html) function.

## 10.6.1 &nbsp; The Metadata Struct
The [`Metadata`](https://doc.rust-lang.org/std/fs/struct.Metadata.html) struct stores known information about a file, such as its permissions, size, and modification times. It is derived from the [`std::sys::unix::fs::FileAttr`](https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/std/sys/unix/fs/struct.FileAttr.html) struct, which contains two fields:

```rust, ignore
pub struct FileAttr {
    stat: stat64,
    statx_extra_fields: Option<StatxExtraFields>,
}
```
`statx_extra_fields` is beyond the scope of this textbook. [`stat64`](https://docs.rs/libc/0.2.71/libc/struct.stat64.html) is a struct defined in the `libc` crate and contains the following members:

```rust, ignore
pub struct stat64 {
    pub st_dev: dev_t,
    pub st_ino: ino64_t,
    pub st_nlink: nlink_t,
    pub st_mode: mode_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt64_t,
    pub st_atime: time_t,
    pub st_atime_nsec: i64,
    pub st_mtime: time_t,
    pub st_mtime_nsec: i64,
    pub st_ctime: time_t,
    pub st_ctime_nsec: i64,
    // some fields omitted
}
```
Most of the `stat64` struct members are beyond the scope of this textbook, except for `st_size` and `st_mode`. The `st_size` member contains the file size, in bytes, and the `st_mode` member contains the file permission bits and the file type, which we discussed in 10.3.1 and 10.2.1, respectively. Table 10.1 in section 10.3.1 shows the file permission bits. We can derive the file type from `st_mode` using the following constants from `libc`:

```rust, ignore
pub const S_IFDIR:  ::mode_t = 16384;
pub const S_IFREG:  ::mode_t = 32768;
pub const S_IFLNK:  ::mode_t = 40960;
pub const S_IFSOCK: ::mode_t = 49152;
```
These two members are used in Web servers and are discussed more in depth in Chapter 11.

## 10.6.2 &nbsp; The Metadata Function
The `std::fs::metadata` function is defined as follows: 

```rust, ignore
pub fn metadata<P: AsRef<Path>>(path: P) -> io::Result<Metadata> {
    fs_imp::stat(path.as_ref()).map(Metadata)
}
```
It calls the [`std::sys::unix::fs::stat`](https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/std/sys/unix/fs/fn.stat.html) function:
```
pub fn stat(p: &Path) -> Result<FileAttr>
```
In short, the `metadata` function takes an argument that can be converted to a reference to a path and returns an `io::Result` type. If the operation is successful, then `Ok(Metadata)` is returned. Otherwise, it returns an  `Err` value. Two common reasons why `metadata` may fail include:

* The user lacks permissions to perform `metadata` on path.
* The path does not exist.

The following example shows how to view a file's metadata. 

```rust, ignore
use std::fs::metadata;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new().create(true).read(true).write(true).open("file.txt")?;
    let file_metadata = metadata("file.txt")?;

    // This returns: "File type: FileType(FileType { mode: 33188 })""
    println!("File type: {:?}", file_metadata.file_type());
    // This returns: "File permissions: Permissions(FilePermissions { mode: 33188 })"
    println!("File permissions: {:?}", file_metadata.permissions());
    /* This returns: "All metadata: Metadata { file_type: FileType(FileType { mode: 33188 }), 
                      is_dir: false, is_file: true, 
                      permissions: Permissions(FilePermissions { mode: 33188 }), 
                      modified: Ok(SystemTime { tv_sec: 1622516353, tv_nsec: 687718590 }), 
                      accessed: Ok(SystemTime { tv_sec: 1622516355, tv_nsec: 343733264 }), 
                      created: Ok(SystemTime { tv_sec: 1622516350, tv_nsec: 404487658 }) } */
    println!("All metadata: {:?}", file_metadata);
    
    Ok(())
}
```