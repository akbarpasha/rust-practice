use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of slice is {}", slice[0]);
    println!("Length of the slice is {}", slice.len());
}

fn print(n: &[f64]){
    for elt in n {
        println!("{}", elt);
    }
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    // len returns the number of elements
    println!("Number of elements in the array {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.

    println!("Borrow a part of the array as slice");
    analyze_slice(&ys[1..4]);

    // Example of empty slice is &[]
    let empty_array: [i32;0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{} : {}", i, xval),
            None => println!("Slow down {} is too far", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);

    // A slice, written [T] without specifying the length, is a region of an array or vector. Since a slice can be any length, slices can’t be stored directly in variables or passed as function arguments. Slices are always passed by reference.

    //A reference to a slice is a fat pointer: a two-word value comprising a pointer to the slice’s first element, and the number of elements in the slice.

    // Whereas an ordinary reference is a non-owning pointer to a single value, a reference to a slice is a non-owning pointer to a range of consecutive values in memory. This makes slice references a good choice when you want to write a function that operates on either an array or a vector.

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let av: &[f64] = &a;

    print(sv); // works on vectors
    print(av); // works on arrays

    // In fact, many methods you might think of as belonging to vectors or arrays are methods defined on slices: for example, the sort and reverse methods, which sort or reverse a sequence of elements in place, are actually methods on the slice type [T].

    //Since slices almost always appear behind references, we often just refer to types like &[T] or &str as “slices,” using the shorter name for the more common concept.

    






}
