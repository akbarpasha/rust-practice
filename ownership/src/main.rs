fn main() {
    // this defines i in the scope of main fn
    // this means if we try to access this var from a different function, it's not going to be accessible.
    let i = 5; 
    println!("{i}");

    // this creates a new scope for the loop. The curly braces start a new scope for the variables defined in the loop
    for i in 1..5 {
        println!("{}", i);
    }

    // You can also create just the scope with curly braces and define variables that are scoped to that
    {
        let name = String::new();
        println!("{}", name);
    }
    // this won't compile and will give an error.
    //println!("{}", name);

    // the concept of scope is not too complicated but we need that knowledge to use
    // references and lifetimes

    // References

    let name = String::from("akbar");
    println!("{name}");

    // we can create a reference to the above variable, which will let us give access to that value 
    // through another variable.
    let n = &name;

    // In the above, n gets assigned the address of the value 

    // while you often need to explicitly dereference references in Rust, certain operations like println! have built-in behavior to automatically dereference when it makes sense to do so. This is why both println!("{}", n) and println!("{}", *n) give the same result.
    println!("{}", n);
    println!("{}", *n);

    // returns the memory address of n
    println!("{:p}", n);

    // Display trait on fmt and how it works with references:

    // let's connect this to the Display trait:

    // The Display trait is implemented for str, not for String directly.
    // When you try to use println! with a String or a reference to a String, Rust looks for a Display implementation.
    // It doesn't find one for String directly, so it tries to dereference.
    // After dereferencing, it has a str, which does have a Display implementation.


    // This dereferencing happens automatically in certain contexts, like with println!, due to a feature called "deref coercion". This is what allows you to use a &String where a &str is expected without explicit dereferencing.

    let name = String::from("akbar");
    let n = &name;
    println!("{}", n);

//     Here's what's happening behind the scenes:

        // n is a &String
        // println! looks for a Display implementation for &String
        // It doesn't find one, so it dereferences &String to String
        // It still doesn't find a Display implementation for String, so it dereferences again
        // Now it has a str, which does have a Display implementation
        // It uses this implementation to print the string

        // This multi-step dereferencing happens automatically, which is why you don't need to write *n explicitly.

// Sized and Unsized types in rust.

// Sized vs. Unsized Types:

// A sized type is one whose size is known at compile time. For example, i32, f64, or any struct with fields of known size.
// An unsized type is one whose size is not known at compile time. These are also called "dynamically sized types" (DSTs).


// Why str is Unsized:

// str represents a string slice of any length. Since strings can be of any length, the compiler doesn't know how much memory a str will occupy at compile time.


// Why We Use &str:

// While we can't use str directly (because its size is unknown), we can use a reference to it: &str.
// A reference to an unsized type is itself a sized type. It consists of two parts:

// A pointer to the data
// Additional information about the unsized type (in this case, the length of the string)



// This combination of pointer and metadata is known as a "fat pointer" and has a known size at compile time.
// How References Make it Sized:

// For &str, the reference contains:

// A pointer to the start of the string data
// The length of the string


// This reference has a fixed size (typically two words: one for the pointer, one for the length), which is known at compile time.


// Example:
//let s: str = "Hello, world!";  // This is not allowed!
let s: &str = "Hello, world!"; // This is fine

// In the first line, we can't create a variable of type str directly because Rust wouldn't know how much stack space to allocate for it.
// In the second line, we're creating a reference to a string slice, which has a known size (typically 16 bytes on a 64-bit system: 8 for the pointer, 8 for the length).


// Lifetimes

let n: &String;
{
    let name = String::from("akbar");
    n = &name; // this would give an error
    
}
println!("{n}");

// error[E0597]: `name` does not live long enough
//    --> src/main.rs:116:9
//     |
// 115 |     let name = String::from("akbar");
//     |         ---- binding `name` declared here
// 116 |     n = &name;
//     |         ^^^^^ borrowed value does not live long enough

// There is no garbage collector in Rust, so any data is owned by a variable and there is only single owner for the data

let first = String::from("Akbar");
let name = first;

println!("{}", first); // this would give error

// in the above example, the data ownership has moved from first to name
// That's why references are useful, as they will give us a view into that data

// Rust has the best error messages!!

// 131 | let first = String::from("Akbar");
//     |     ----- move occurs because `first` has type `String`, which does not implement the `Copy` trait
// 132 | let name = first;
//     |            ----- value moved here
// 133 |
// 134 | println!("{}", first); // this would give error
//     |                ^^^^^ value borrowed here after move
//     |
//     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//     |
// 132 | let name = first.clone();
//     |                 ++++++++


// the above is only applicable for data created on heap.
// for primitives, it's not an issue.

let score = 50;
let points = score;
println!("{}", score); // this will print

// the above value is being copied, not moved.

// by default if the type doesn't implement Copy trait then when you assign it, it will move instead of copy.
// for String data type, we don't have Copy trait implemented, but we have Clone and the difference is that 
// that needs to be called explicitly

}
