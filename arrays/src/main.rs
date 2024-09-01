
use std::str;
fn main() {
    
// Rust has three types for representing a sequence of values in memory:

// The type [T; N] represents an array of N values, each of type T. An array’s size is a constant determined at compile time and is part of the type; you can’t append new elements or shrink an array.

// The type Vec<T>, called a vector of Ts, is a dynamically allocated, growable sequence of values of type T. A vector’s elements live on the heap, so you can resize vectors at will: push new elements onto them, append other vectors to them, delete elements, and so on.

// The types &[T] and &mut [T], called a shared slice of Ts and mutable slice of Ts, are references to a series of elements that are a part of some other value, like an array or vector. You can think of a slice as a pointer to its first element, together with a count of the number of elements you can access starting at that point. A mutable slice &mut [T] lets you read and modify elements, but can’t be shared; a shared slice &[T] lets you share access among several readers, but doesn’t let you modify elements.

// writing arrays

let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
let taxonomy = ["Animalia", "Anthropoda", "Insecta"];

assert_eq!(lazy_caterer[3], 7);
assert_eq!(taxonomy.len(), 3);

// This will fill in the length of the array with the specified value
let mystring = ["akbar"; 10];

for n in mystring{
    println!("{}", n);
}

// You’ll see this syntax used for fixed-size buffers: [0u8; 1024] can be a one-kilobyte buffer, filled with zeros. Rust has no notation for an uninitialized array.

// You’ll see this syntax used for fixed-size buffers: [0u8; 1024] can be a one-kilobyte buffer, filled with zeros. Rust has no notation for an uninitialized array.


let mut buffer = [0u8; 1024];

let message = b"Hello rust buffer";
buffer[..message.len()].copy_from_slice(message);

// The .. syntax:
// The .. is part of Rust's range syntax. In this context, it's creating a slice of the buffer.

// buffer[..] would mean "the entire buffer"
// buffer[..n] means "from the start of the buffer up to (but not including) index n"
// buffer[n..] would mean "from index n to the end of the buffer"
// buffer[m..n] would mean "from index m up to (but not including) index n"


// message.len():
// This gets the length of the message byte slice.
// buffer[..message.len()]:
// This creates a slice of the buffer from the start (index 0) up to the length of the message. It's essentially saying "give me a slice of the buffer that's the same length as the message".
// .copy_from_slice(message):
// This method copies the contents of message into the slice we just created.

println!("{}", str::from_utf8(&buffer[..buffer.len()]).unwrap());

for i in 0..10 {
    buffer[message.len() + i] = i as u8;
}

println!("Counter values {:?}", &buffer[message.len()..message.len() + 10]);

println!("{:?}", &buffer);

println!("{}", str::from_utf8(&buffer).unwrap());


let secret_message = b"Rust is awesome";
let shift = 3;

//for (i, &byte) in secret_message.iter().enumerate(){
//    let position = message.len() + 10 + i;
//    buffer[position] = (byte.wrapping_add(shift) - b'A') % 26 + b'A';
    // Caeser cipher encryption above:

    // byte: This is the current character from the secret message we're encrypting.
    // byte.wrapping_add(shift): This adds the shift value (3 in our example) to the byte value of the character. wrapping_add is used to handle potential overflow, though it's not strictly necessary in this case.
    // - b'A': We subtract the ASCII value of 'A' (which is 65). This effectively maps 'A' to 0, 'B' to 1, 'C' to 2, and so on.
    // % 26: This performs a modulo operation with 26 (the number of letters in the alphabet). It ensures our result stays within the range 0-25, even after shifting.
    // + b'A': We add back the ASCII value of 'A'. This maps our 0-25 result back to the ASCII values for 'A' through 'Z'.
    // The result is then stored in buffer[position].

    // Let's walk through an example:

    // Say we're encrypting the letter 'R' with a shift of 3.
    // The ASCII value of 'R' is 82.
    // 82 + 3 = 85
    // 85 - 65 ('A') = 20
    // 20 % 26 = 20 (no change in this case)
    // 20 + 65 = 85, which is the ASCII value for 'U'

    // So 'R' gets encrypted to 'U'.
//}

//print the encrypted message
//let encrypted = str::from_utf8(&buffer[message.len() + 10..message.len() + 10 + secret_message.len()]).unwrap();
//println!("{}", encrypted);

//All methods are on slices
// The useful methods you’d like to see on arrays—iterating over elements, searching, sorting, filling, filtering, and so on—are all provided as methods on slices, not arrays. But Rust implicitly converts a reference to an array to a slice when searching for methods, so you can call any slice method on an array directly:

let mut chaos = [3, 5, 4, 1, 2];
chaos.sort();
assert_eq!(chaos, [1,2,3,4,5]);

// The useful methods you’d like to see on arrays—iterating over elements, searching, sorting, filling, filtering, and so on—are all provided as methods on slices, not arrays. But Rust implicitly converts a reference to an array to a slice when searching for methods, so you can call any slice method on an array directly

// A vector Vec<T> is a resizable array of elements of type T, allocated on the heap.

// There are several ways to create vectors. The simplest is to use the vec! macro, which gives us a syntax for vectors that looks very much like an array literal:

let mut primes = vec![2, 3, 5, 7];
assert_eq!(primes.iter().product::<i32>(), 210);


// You can also build a vector by repeating a given value a certain number of times, again using a syntax that imitates array literals:

fn new_pixel_buffer(rows: usize, cols: usize)-> Vec<u8> {
    vec![0; rows * cols]
}

// The vec! macro is equivalent to calling Vec::new to create a new, empty vector and then pushing the elements onto it, which is another idiom:

let mut pal = Vec::new();
pal.push("step");
pal.push("on");
pal.push("no");
pal.push("pets");

assert_eq!(pal, ["step", "on", "no", "pets"]);

// Another possibility is to build a vector from the values produced by an iterator:
let v:Vec<i32> = (0..5).collect();
assert_eq!(v, [0,1,2,3,4]);

// A Vec<T> consists of three values: a pointer to the heap-allocated buffer for the elements, which is created and owned by the Vec<T>; the number of elements that buffer has the capacity to store; and the number it actually contains now (in other words, its length). When the buffer has reached its capacity, adding another element to the vector entails allocating a larger buffer, copying the present contents into it, updating the vector’s pointer and capacity to describe the new buffer, and finally freeing the old one.

// If you know the number of elements a vector will need in advance, instead of Vec::new you can call Vec::with_capacity to create a vector with a buffer large enough to hold them all, right from the start; then, you can add the elements to the vector one at a time without causing any reallocation. The vec! macro uses a trick like this, since it knows how many elements the final vector will have. Note that this only establishes the vector’s initial size; if you exceed your estimate, the vector simply enlarges its storage as usual.

let mut v: Vec<i32> = Vec::with_capacity(2);
assert_eq!(v.len(), 0);
assert_eq!(v.capacity(), 2);

let vr = &v;
println!("{:p}", vr);

v.push(1);
v.push(2);

assert_eq!(v.len(), 2);
assert_eq!(v.capacity(), 2);

v.push(3);
println!("{}", v.capacity()); // capacity is 4, len is 3
let vr = &v;
println!("{:p}", vr);


let mut v = vec![10, 20, 30, 40, 50];
v.insert(3, 35);
assert_eq!(v, [10, 20, 30, 35, 40, 50]);
println!("{:?}", v);

v.remove(1);
assert_eq!(v, [10, 30, 35, 40, 50]);
println!("{:?}", v);

// You can use the pop method to remove the last element and return it. More precisely, popping a value from a Vec<T> returns an Option<T>: None if the vector was already empty, or Some(v) if its last element had been v:

let mut v = vec!["Snow Puff", "Glass Gem"];
println!("{:?}", v);
assert_eq!(v.pop(), Some("Glass Gem"));
println!("{:?}", v);
assert_eq!(v.pop(), Some("Snow Puff"));
println!("{:?}", v);
assert_eq!(v.pop(), None);
println!("{:?}", v);


}
