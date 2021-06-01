# Chapter 5: Optimizing Program Performance
---
### 5.1 Capabilities and Limitations of Optimizing Compilers
...

Compilers must be carefult to apply only _safe_ optimizations to a program, meaning that the resulting program will have the exacts same behavior as would an unoptimized version for all possible cases the program may encounter, up to the limits of the guarantees by the rust language. Constraining the compiler to perform only safe optimizations eliminates possible sources of undesired run-time behavior, but it also means that the programmer must make more of an effort to write programs in a way that the compiler can then transform into efficient machine-level code. To appreciate the challenges of deciding which program transformations are safe or not, consider the following two procedures:

```rust,no_run
fn twiddle1(xp: *mut i64, yp: *mut i64) {
    unsafe {
        *xp += *yp;
        *xp += *yp;
    }
}

fn twiddle2(xp: *mut i64, yp: *mut i64) {
    unsafe {
        *xp += 2* *yp;
    }
}
# fn main() {}
```

At first glance, both procedures seem to have identical behavior. They both addd the value stored at the location designated by pointer yp to that designated by pointer xp. On the other hand, function witddle2 is more efficient. It requires only three memory references (read *xp, read *yp, write *xp), whereas twiddle1 requires six (two reads of *xp, two reads of *yp, and two writes of *xp). Hence, if the compiler is given procedure twiddle1 to compile, one might think it could generate more efficient code based on the computations performed by twiddle2.

Consider, however, the case in which xp and yp are equal. Then function twiddle1 will perform the following computations:

```rust,no_run
# fn twiddle1(xp: *mut i64, yp: *mut i64) {
#    unsafe {
*xp += *yp;
*xp += *yp;
#   }
# }
# fn main() {}
```

The result will be that the value at xp will be increased by a factor of 4. On the other hand, function twiddle2 will perform the following computation.:

```rust,no_run
# fn twiddle2(xp: *mut i64, yp: *mut i64) {
#    unsafe {
 *xp += 2* *yp;
#   }
# }
# fn main() {}
```

The result will be that hte value at xp will be increased by a factor of 3. The compiler knows nothing about how twiddle1 will be called, and so it must assume that the arguments of xxp and yp can ve equal. It therefor cannot generagte code in the styple of twiddle2 as an optimized version of twiddle1.

The case where two pointers may designate the same memory location is know as _memory aliasing_. In rust, memory aliasing is mostly considered unsafe since it allows multiple variables to own a place in memory. In performing only safe optimizations, the compiler must assume that different pointers may be aliased. As another example , for a program with pointer variables p and q, consider the following code sequence:

```rust,no_run
# fn main() {
let x = 1000;
let y = 3000;
# let mut t1 = 0;
# let q: *mut i32 = &mut t1;
# let p: *mut i32 = &mut t1;
# unsafe{
*q = y;   /* 3000 */
*p = x;   /* 1000 */
t1 = *q;  /* 1000 or 3000 */
# };
# }
```

The value computed for t1 depends on whether or not pointers p and q are aliased --- if not, it will equal 3,000, but if so it will equal 1,000. This leads to one of the major _optimization blockers_, aspects of programs that can severly limit the opportunities for a compiler to generate optimized code. If a compiler cannot determin wheter or not two pointers may be aliased, it must assume that either case is possible, limiting the set of possible optimizations.

`Practice 5.1`
The following problem illustrates the way memoyr aliasing can cause unexpectede program behavior. Consider the following procedure to swap two values:

```rust,no_run
fn swap(xp: *mut i64, yp: *mut i64) {
    unsafe {
        *xp = *xp + *yp;
        *yp = *xp - *yp;
        *xp = *xp - *yp;
    }
}
# fn main() {}
```
If this procedure is called with xp equal to yp, wat effect will it have?

...

A second optimization blocker is due to function calls. As an example, consider the following two procedures:


```rust,no_run
fn f() -> i64{ todo!() }
fn func1() -> i64 {
    f() + f() + f() + f()
}

fn func2() -> i64 {
    4*f()
}
# fn main() {}
```

It might seem at first that both compute the same result but with func2, calling f only once, whereas func1 calls it four times. It is tempting to generate code in the styple of func2 when given func1 as the source.

Consider, however, the following code for f:

```rust,no_run
static mut COUNTER: i32 = 0;
# fn main() {
fn f() -> i64 {
#        unsafe {
	let count = COUNTER.into();
	COUNTER += 1;
	count.into()
#        }
}
# }
```

This function has a _side effect_ --- it modifies some part of of the global program state. Changing the number of times it gets called changes the program behavior. In particular, a call to func1 would return 0 + 1 + 2 + 3 = 6, whereas the call to func2 would return 4 * 0 = 0, assuming both started with global variable counter set to zero.

Most compilers do not try to determine whether a function is free of side effects and hence is a candidate for optimizations such as those attempted in func2. Instead, the compiler assumes  the worse case and leaves function calls intact.

---
### 5.2 Expressing Program Performance

...

```rust,no_run
# fn main() {}
fn psum1(a: Vec<f32>, p: &mut Vec<f32>, n: usize) {
    p[0] = a[0];
    for i in 1..n {
        p[i] = p[i-1] + a[i];
    }
}

fn psum2(a: Vec<f32>, p: &mut Vec<f32>, n: usize) {
    p[0] = a[0];
    for i in (1..n-1).step_by(2) {
        let mid_val: f32 = p[i-1] + a[i];
        p[i] = mid_val;
        p[i+1] = mid_val + a[i+1];
    }
    if n % 2 == 0 {
        p[n-1] = p[n-2] + a[n-1];
    }
}
```
`Figure 5.1`

...

---
### 5.3 Program Example

To demonstrate how an abstract program can be systematically transformed into more efficient code, we will use a running example bacsed on the vector data structure shown in figure 5.3. A vector is represneted with two blocks of memory: the header and the data array. The header is a structure declared as follows:

The declaration uses longs (`i64`) as the default type of the elements.

Figure 5.4 shows some baic procedures for generating vecotrs, accesing vecot elements, determining the length of a vector, and a few other things. An important feature to not is that _get\_vec\_element_, the vecotr access function, performs bounds checking for every vector reference . This code is similar to the array representations used in many other languages, including Java. Bounds checking reduces the chances of program error, but it can also slow down program execution.

```rust,no_run
{{#include ../code/main.rs:1:58}}
```
Figure 5.4

As an optimization example, consider the code shown in Figure 5.5, which combines all of the elements in a vector into a single value according to some operation.

In our presentation, we will proceed through a series of transformations of the code, writing different versions of the combining function. 

...

```rust,no_run
{{#include ../code/main.rs:61:68}}
```
Figure 5.5


---
### 5.4 Eliminating Loop Inefficiencies

Observe that procedure `combine1`, as shown in Figure 5.5, calls function vec_length as teh test condition of the for loop. Recall from our discussion of how to translate code containing loops into machine-level programs (Section 3.6.7) that the test condition must be evaluated on every iteration of the loop. On the other hand, the length of the vector does not change as teh loop proceeds. We could therefore compute the vector length only one an duse this value inour test condition.

Figure 5.6 shows a modified version called `combine2`. It calls vec_length at the beginning and assigns the result to a local variable length. This transformation has noticable effect on the overfall performance for some data types and operations, and minimal or even none for others. In any case, this transformations is required to eliminate inifficiencies that would become bottlenexks as we attempt further optimizations.

```rust,no_run
{{#include ../code/main.rs:70:78}}
```
Figure 5.6

...

---
### 5.5 Reducing Procedure Calls

As we have seen, procedure calls can incur overhead and also block most forms of program optimization we can see in the code for combine2 (Figure 5.6) that get\_vec\_element is called on every loop iteration to retrieve the next vector element. This function checks the vector index `i` against the loop bounds with every vector reference, a clear source of inefficiency. Bounds checking might be a useful feature when dealing with arbitrary array accesses, but a simle analysis of the code for `combine2` shows that all references will be valid.

```rust,no_run
{{#include ../code/main.rs:80:87}}
```
Figure 5.9

Suppose instead that we use the fucntion get\_vec\_start (Figure 5.4) to our abstract data type. This function returns the starting address of the data array, as showin in Figure 5.9. We could then write the procedure shown as combine3 in this ficugrfe, having no function calls in the inner loop. Rather than making a function call to retrieve each vector element, it accessses the array directl. A purist might stay that this transformation seriously impairs the program modularity. In principle, the user of the vector abstract data type should not even need to know that the vector contents are stored as an array, rather than as some other data structure such as a linked list. A more pragmatic programmer would argue that this transformation is a necessary step towrad achieveing high-performance results.

...

---
### 5.6 Eliminating Unneeded Memory References

The code for `combine3` accumulates the value being computed by the combining operation at the location designated by the pointer `dest`. This attribute can be seen by examining the assembly code generated by the inner loop of the combiled code

...

We can eliminate this needless reading and writing of memory by rewriting the code in the style of `combine4` inf Figure 5.10. We introduce a temporary value acc that is sued in the loop to accumulate the computed values. The result is stored at dest only after the loop has been completed.

...

We see a signification improvement in the program performance, as will be seen in Section 5.10.

```rust,no_run
{{#include ../code/main.rs:89:97}}
```
Figure 5.10

...

---
### 5.7 Understanding Modern Processors (unchanged)
---
### 5.8 Loop Unrolling

Loop unrolling is a program transformation that reduced the number of iterations for a loop by increasing the number of elements computed on each iteration. We saw an example of this with the function `psum2` (figure 5.1), where each iteration computes two elements of the prefix sum, thereby halving the total number of iterations required. Loop unrolling can improve performance in two ways. First, it reduces the numbr of operations that do not contribute directly to the program result, such as loop indexing and conditional branching. Second, it exposes ways in whiich we can further transform the code to reduce the number of operations in the critical paths of the overall computation. In this section, we will examine simple loop unrolling, without any further transformations.

Figure 5.16 shows a version of our combining code using what we will refer to as "2 x 1 loop unrolling." This frist loop steps through the array of two elements at a time. That is, the loop index `i` is incremented by 2 on each iteration, and the combining operation is applied to array elements `i` and `i + 1` in a single iteration.

...

```rust,no_run
{{#include ../code/main.rs:100:117}}
```
Figure 5.16

...

---
### 5.9 Enchancing Parallelism

...

#### 5.9.1 Multiple Accumulators

...

Figure 5.21 shows code that uses this method. It uses both two-way loop unrolling, to combine more elements per iteration, and two-way parallelism, accumulating elements with even indices in verable acc0 and elements with odd indicies in veraible acc1. We therefore refer to this as "2 x 2 loop unroling." Ass before, we include a second loop to accumulate any remaining array elements for the case where teh vector length is not a multiple of 2. We then appply combinding code opeation to acc0 and acc1 to compute the final result.

```rust,no_run
{{#include ../code/main.rs:120:140}}
```
Figure 5.21

...

#### 5.9.2

We now explore another way to break the sequential dependencies and thereby improve performance beyond the latency bound. We saw that the `k x 1` loop unrolling of `combine5` did not change the set of operations performed in combining the vector elements to form their sum or producet. By a very small change in the code, however, we can fundamentally change the way the combining is performed, and also greatly increase the performance. 

Figure 5.26 shows a function `combine7` that differs from the unrolled code of `combine5` (Figure 5.16) only in the way the elements are combined in the inner loop. In `combine5`, the combining is performed by the statement

{{#include ../code/main.rs:109}}

while in `combine7`, it is performed by the statement

{{#include ../code/main.rs:152}}

differing only in how two parentheses are placed. We call this a reassociation transformation, because the parenthesess shif the oder in which the vector elements are combined with the accumulated value acc, yielding a form of loop unrolling we refer to as "2 x 1a."

... 

```rust,no_run
{{#include ../code/main.rs:143:161}}
```
Figure 5.26

---
### 5.10 Summary of Results for Optimizing Combining Code (unchanged)

...

## Rust results summary

We'd expect the 7 combine functions to get faster and faster with the exception of `combine7` which we expect to have similar performance to `combine5`. As we will see, however, this isn't exactly the case. We ran 2 sets of benchmarks on all 7 functions. The first set ran without taking advantage of rust's optimizaitions while the second ran with opt-level = 3. Here, we will take a look at all of these and discuss possibilities as to why some of the results are inconsistent with what we might expect.

Each benchmark creates a vector of size 10,000 where each element, `i` holds the value `i * i`. Then it calls one of the combine functions to determine the sum of the vector.

#### Unoptimized:

<style>
r { color: Red }
o { color: Orange }
g { color: Green }
</style>

Here, two of the combine functions are not what would've been expected: `combine4` and `combine6`. We'd expect `combine4` to be faster than `combine3`, and we'd expect `combine6` to be the fastest function. The three main culprits of these discrepencies are the difference in languages, machine architecture, and compilers.

These functions were originally written for optimizing C code, and rust being designed with a different purpose in mind, their compilation processes follow a different set of rules that allow for different binaries that GCC or another C compiler wouldn't give.

For `combine6`, there's a chance the loop unrolling couldn've slown down execution because of different compilers or different architecture. These were originally run on an Intel Core i7 Haswell processor when benchmarked in C. With rust, they were run on an AMD Ryzen 7 processor. Including the reseach done suggesting that loop unrolling in AMD processors perform worse than on Intel processors, the cache size to data size ratio may also have played a factor.

It is worth noting that rust does provide the attribute-like macro `unroll_for_loops` in the unroll crate to allow the rust compiler to handling the unrolling as it sees fit. Speedups when using this crate can match the optimized version of `combine4`.


> combine1          time:   [176.00 us <r>176.69 us</r> 177.53 us]  
combine2          time:   [173.54 us <r>175.12 us</r> 176.90 us]  
combine3          time:   [158.76 us <r>164.99 us</r> 170.80 us]  
combine4          time:   [168.36 us <r>173.15 us</r> 178.22 us]  
combine5          time:   [24.625 us <r>24.635 us</r> 24.647 us]  
combine6          time:   [26.494 us <r>26.502 us</r> 26.511 us]  
combine7          time:   [25.117 us <r>25.126 us</r> 25.137 us]  


#### Optimized:

Running the benchmarker with the optimizing function, the only odd function is `combine4` which is orders of magnitude faster than all of the other functions. This has to do with how rust compiles this code. For whatever reason, the rust compiler deemed it a good idea to take advantage of XMM registers which are a kind of 128 bit register that can be used to perform simultaneous operations on 2 longs. Looking at the difference between `combine3` and `combine4`, the change that most likely allowed this optimization to be able to take place is the lack of needing to dereference an accumulator before adding to it. Since accumulation is happening sequentially and within a local variable, rustc (the rust compiler), was smart enough to figure out that it could use these XMM registers.

>combine1                time:   [4.7249 us <r>4.7416 us</r> 4.7762 us]  
combine2                time:   [4.7097 us <r>4.7107 us</r> 4.7119 us]  
combine3                time:   [2.4628 us <r>2.4646 us</r> 2.4670 us]  
combine4                time:   [663.91 ns <r>664.95 ns</r> 666.52 ns]  
combine5                time:   [2.3855 us <r>2.3861 us</r> 2.3867 us]  
combine6                time:   [1.6023 us <r>1.6033 us</r> 1.6047 us]  
combine7                time:   [2.3611 us <r>2.3624 us</r> 2.3642 us]  

---

