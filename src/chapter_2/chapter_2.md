# Chapter 2: Representing and Manipulating Information

---
### 2.1 Information Storage 
##### (starting p. 44, last 2 paragraphs)

...

A third case where byte ordering becomes visible is when programs are written that circumvent the normal type system. In rust, this can be done using generics to allow an object to be references according to a different data type from which it was created. Such coding tricks are strongly discouraged for most application programming, but they can be quite useful and even necessary for system-level programming.

Figure 2.4 shows rust code that uses casting to access and print the byte representations of different program objects. We use a generic type T that has the Serialize trait to read the bytes of a variable. The serialized value references a sequence of bytes where each byte is considered to be a non-negative integer. The first routine *show\_bytes* is given the value that needs to be serialized. The rust formatting directive *\{:02x\}* indicates that an integer should be printed in hexadecimal with at least 2 digits.

```rust,no_run
# use std::mem::size_of;
fn show_bytes(val: *mut u8, length: isize) {
    for i in 0..length {
        print!(" {:02x}", unsafe { *(val.offset(i)) });
    }
    print!("\n");
}

fn show_int(x: i32) {
    show_bytes(&x as *const _ as *mut u8, size_of::<i32>() as isize);
}

fn show_float(x: f32){
    show_bytes(&x as *const _ as *mut u8, size_of::<f32>() as isize);
}

fn show_pointer<T>(x: *const T){
    show_bytes(&x as *const _ as *mut u8, size_of::<* const T>() as isize);
}
# fn main() {}
```
Figure 2.4

Procedures show_int, show_float, and show_pointer demonstrate how to use procedure  show_bytes to print the byte representations of rust program objects of type i32, f32, and *const T, respectively. Observe that they pass show_bytes a reference &x to the argument x, casting to a char pointer. The intermediate cast ("as _") asks the compiler to the work of figuring out what to cast the type to since you can't cast directly change the type a pointer points to in rust. Ultimately this cast lets the compiler know that the program should consider the pointer to be a sequence of bytes rather than to an object of the original data type. This pointer will then be to the lowest byte address occupied by the object.

The procedures use the rust size_of function to determine the number of bytes used by the object. In general, the expression size_of::<T\>() returns the number of bytes required to store an object of type T. Using sizeof rather than a fixed value is one step toward writing code that is portable across different machine types.

...

```rust
# use std::mem::size_of;
# fn show_bytes(val: *mut u8, length: isize) {
#     for i in 0..length {
#        print!(" {:02x}", unsafe { *(val.offset(i)) });
#     }
#     print!("\n");
# }
# fn show_int(x: i32) {
#    show_bytes(&x as *const _ as *mut u8, size_of::<i32>() as isize);
# }
# fn show_float(x: f32){
#    show_bytes(&x as *const _ as *mut u8, size_of::<f32>() as isize);
# }
# fn show_pointer<T>(x: *const T){
#    show_bytes(&x as *const _ as *mut u8, size_of::<* const T>() as isize);
#}
fn main() { 
    test_show_bytes(100);
}

fn test_show_bytes(val: i32) {
    let ival: i32 = val;
    let fval: f32 = val as f32;
    let pval: *const i32 =  &val as *const i32;
    show_int(ival);
    show_float(fval);
    show_pointer(pval);
}
```
Figure 2.5

...

> New to Rust? -- Formatting printing with print!
> 
>  The print! function (along with its cousins `println!`, `eprint!`, and `eprintln!`) provide a way to print information with considerable control over the formatting details. The first argument is a *format string*, while any remaining arguments are values to be printed. Within the format string, each character sequence enclosed in `{...}` indicates how to format the next argument. Typical examples include {} which stringifies a value if possible and {:?} which display the values in debugging mode if the type has the Debug trait implemented.

> New to Rust? -- Pointer creation and dereferencing 
> 
> At the lowest level, borrowed references in rust (`&`) are equivalent to C pointers. So why would the type of x in  show_pointer have to be `*const T` instead of just using `&`? This is because rust enforces memory safety at compile time. A result of the memory safety is that a spot in memory can only be owned by one variable at any given time, but it allows a function to borrow ownership of a variable by placing a `&` in before the variable name. But in order to interpret it as a pointer and not a reference to the variable, you have to cast it to a constant pointer `*const T` where T is a generic type. You can also cast it to a specific type of pointer if you know the type (that is, if you wanted a pointer to an int, it would be `*const i32`).


> **Aside** Generating an ASCII Table
> 
> You can display a table showing the ASCII character code by executing the command `man ascii`,

`Practice 2.5`

Consider the following three calls to *show_bytes*:
```rust
# #![allow(overflowing_literals)]
# use std::mem::size_of;
# fn show_bytes(val: *mut u8, length: isize) {
#     for i in 0..length {
#        print!(" {:02x}", unsafe { *(val.offset(i)) });
#     }
#     print!("\n");
# }
#
# fn main() {
let val: i32 = 0x87654321;
let valp = &val as *const _ as *mut u8;
show_bytes(valp, 1);   /* 1. */
show_bytes(valp, 2);   /* 2. */
show_bytes(valp, 3);   /* 3. */
#}
```
Indicate the values that will be printed by each call on a little-endian machine and on a big-endian machine:

1. Little endian: __________  &emsp;&emsp;&emsp; Big endian: __________
2. Little endian: __________  &emsp;&emsp;&emsp; Big endian: __________
3. Little endian: __________  &emsp;&emsp;&emsp; Big endian: __________

---

`Practice 2.10`
```rust,no_run
fn inplace_swap(x: *mut i32, y: *mut i32) {
	unsafe {
	    *y = *x ^ *y; /* Step 1 */
	    *x = *x ^ *y; /* Step 2 */
	    *y = *x ^ *y; /* Step 3 */
    }
}
# fn main() {}
```
As the name implies, we claim that the effect of this procedure is to swap the values stored at the locations denoted by pointer variables *x* and *y*. Note that unlike the usual technique for swapping two values, we do not need a third location to temporarily store one value while we are moving the other. There is no performance advantage to this way of swapping; it is merely an intellectual amusement.

Starting with values *a* and *b* in the locations pointed to by *x* and *y*, respectively, fill in the table that follows, giving the values stored at the two locations after each step of the procedure. Use the properties of ^ to show that the desired effect is achieved. Recall that every element is its own additive inverse (that is, *a* ^ *a* = 0).

| Step | *x | *y |
|----------|:-------------:|:------:| 
| Initially | a | b |
| Step 1 | ________ | ________ |
| Step 2 | ________ | ________ |
| Step 3 | ________ | ________ |


> New to Rust? Unsafe Rust
>  A powerful feature of rust that separates it from other languages is that it enforces memory safety guarantees at compile time. However sometimes in systems programming you need to perform a task that isn't memory safe. That's where the `unsafe` keyword comes in. Unsafe rust works exactly like normal rust without the guarantee of memory safety. In inplace_swap, we see it's used to dereference and mutate a mutable pointer. 

`Practice 2.11`
Armed with the function *inplace_swap* from problem 2.10, you decide to write code that will reverse the elements of an array by swapping elements from opposite ends of the array, working toward the middle. You arrive at the following function:
```rust,no_run
# fn inplace_swap(x: *mut i32, y: *mut i32) {
#	unsafe {
#	    *y = *x ^ *y; /* Step 1 */
#	    *x = *x ^ *y; /* Step 2 */
#	    *y = *x ^ *y; /* Step 3 */
#    }
# }
fn reverse_array(a: &mut Vec<i32>, cnt: usize) {
	let mut last = cnt-1;
    for first in 0..=cnt/2 {
        inplace_swap(&mut a[first], &mut a[last]);
        last -= 1
    }
}
# fn main() {}
```
When you apply the function to an array containing elements 1, 2, 3, and 4, you find the array now has, as expected, elements 4, 3, 2, and 1. When you try it on an array with elements 1, 2, 3, 4, and 5, however you are surprised to see that the array now has elements 5, 4, 0, 2, and 1. In fact, you discover that the code always works correctly on arrays of  even length, but it sets the middle element  to 0 whenever the array has odd length.

1. For an array of odd length *cnt = 2k + 1*, what are the values of variables *first* and *last* in the final iteration of function *reverse_array*?
2. Why does this call to function *inplace_swap* set the array element to 0?
3. What simple modification to the code for *reverse_array* would eliminate this problem? 
---
### 2.2 Integer Representations
Rust supports a variety of *integral* data types---ones that represent finite ranges of integers. These are shown in Figure 2.9, along with the ranges of values the can have for "typical" 64-bit programs. Each type can specify a size with a number telling how many bytes they type has as well as an indication of whether the represented numbers are all non-negative (declared as `u`), or possibly negative (`i`). 

| Rust data type | Minimum | Maximum |
|----------|-------------:|------:| 
| i8 | -128 | 127 |
| u8  | 0 | 255 |
| i16 | -32,768 | 32,767 |
| u16 | 0 | 65,535 |
| i32 | -2,147,483,648 | 2,147,483,647 |
| u32  | 0 | 4,294,967,295 |
| i64 | -9,223,372,036,854,775,808 | 9,223,372,036,854,775,807 |
| u64 | 0 | 18,446,744,073,709,551,615 |

Figure 2.9 Typical ranges for Rust integral data types for 64-bit programs

---

2.2.5 **Signed versus Unsigned in Rust**

As indicated in Figure 2.9, Rust supports both signed and unsigned arithmetic for all of its integer data types. Generally, most numbers are signed by default. For example, when declaring a constant such as 12345 or 0x1A2B, the value is considered signed. Adding an '_u#' as a suffix creates an unsigned constant.; for example, 12345_u32 or 0x1A2B_u32.

Rust allows covertion between unsigned and signed. Although rust does not specify how this conversion should be made, most systems follow the rule that the underlying bit representation does not change. This rule has the effect  of applying the function *U2T<sub>w<sub>* when converting from unsigned to signed, and *T2U<sub>w<sub>* when converting from signed to unsigned, where w is the number of its for the data type.

Conversions can only happen due to explicit casting, such as in the following code:

```rust
# fn main() {
let tx;
let ty = 0x5678;
let ux = 0x1234_u32;
let uy: u32;

tx = ux as i32;
uy = ty as u32;
println!("tx: 0x{:x}, uy: 0x{:x}", &tx, &uy);
# }
```

Other languages, such as C, allow you to implicitly cast from signed to unsigned simply by setting the value equal to a value with a different type.

---

2.2.6 **Expanding the Bit Representation of a Number**

One common operation is to convert between integers having different word sizes while retaining the same numeric value. Of course, this may not be possible when the destination data type is too small to represent the desired value. Converting from a smaller to a larger data type, however should always be possible.

<!-- To convert an unsigned number to a larger data type, we can simply add leading zeros to the representation; this operation is known as *zero extens -->

...

As an example, consider the following code:

```rust
# #![allow(overflowing_literals)]
# use std::mem::size_of;
# fn show_bytes(val: *mut u8, length: isize) {
#     for i in 0..length {
#        print!(" {:02x}", unsafe { *(val.offset(i)) });
#     }
#     print!("\n");
# }
#
# fn main() {
let sx = -12345_i16;
let usx = sx as u16;
let x = sx as i32;
let ux = usx as u32;
print!("sx = {}:", &sx);
show_bytes(&sx as *const _ as *mut u8, size_of::<i16>() as isize);
print!("usx = {}:", &usx);
show_bytes(&usx as *const _ as *mut u8, size_of::<u16>() as isize);
print!("x = {}:", &x);
show_bytes(&x as *const _ as *mut u8, size_of::<i32>() as isize);
print!("ux = {}:", &ux);
show_bytes(&ux as *const _ as *mut u8, size_of::<u32>() as isize);
# }
```
---
### 2.3 Integer Arithmetic(unchanged)
---
### 2.4 Floating Point (unchanged)
---


