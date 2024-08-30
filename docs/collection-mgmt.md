# Building a Simple Collection Management System in Rust

In this blog post, we'll explore how to build a simple collection management system using Rust. We'll create a program that allows users to add items to a collection, update their quantities, and list all items. This project will demonstrate the use of structs, implementations, HashMaps, and basic input/output operations in Rust.

## Setting Up the Project

First, let's start by importing the necessary modules:

```rust
use std::io::{self, Write};
use std::collections::HashMap;
```

We're using `std::io` for input/output operations and `std::collections::HashMap` to store our items.

## Defining the Data Structures

We'll define two structs: `Item` and `Collection`:

```rust
struct Item {
    _name: String,
    _quantity: u8,
}

struct Collection {
    _items: HashMap<String, Item>,
}
```

The `Item` struct represents an individual item with a name and quantity. The `Collection` struct uses a `HashMap` to store items, with the item name as the key and the `Item` struct as the value.

## Implementing Collection Methods

Now, let's implement methods for our `Collection` struct:

```rust
impl Collection {
    fn new() -> Self {
        Collection {
            _items: HashMap::new(),
        }
    }

    fn add_item(&mut self, name: String, quantity: u8) {
        let item = Item {
            _name: name.to_string(),
            _quantity: quantity,
        };

        self._items.insert(name.to_string(), item);
        println!("Added an item {} and quantity {}", name, quantity);
    }
    
    fn update_item(&mut self, name: String, quantity: u8) {
        if let Some(item) = self._items.get_mut(&name) {
            item._quantity = quantity;
            println!("Updated item: {} and quantity {}", name, quantity);
        } else {
            println!("No item in the collection");
        }
    }
    
    fn list_item(&self) {
        if self._items.is_empty() {
            println!("There are no items in the list");
        } else {
            for item in self._items.values() {
                println!("Added item: {} and quantity: {}", item._name, item._quantity);
            }
        }
    }
}
```

Let's break down these methods:

1. `new()`: This is a constructor that creates a new `Collection` with an empty `HashMap`.
2. `add_item()`: Adds a new item to the collection.
3. `update_item()`: Updates the quantity of an existing item.
4. `list_item()`: Lists all items in the collection.

## The Main Function

Finally, let's implement the `main()` function to tie everything together:

```rust
fn main() {
    let mut collection = Collection::new();

    loop {
        println!("1. Add an item");
        println!("2. Update an item");
        println!("3. List an item");
        println!("4. Exit");

        print!("Enter your choice: ");

        io::stdout().flush().expect("Failed to flush stdout");

        let mut take_input: String = String::new();

        io::stdin().read_line(&mut take_input).expect("Failed to read line");

        let choice: u8 = take_input.trim().parse().expect("Failed to convert to integer");

        match choice {
            1 => collection.add_item(String::from("Apple"), 5),
            2 => collection.update_item(String::from("Apple"), 8),
            3 => collection.list_item(),
            4 => break,
            _ => println!("Failed to recognize the choice"),
        }
    }
}
```

This function creates a new `Collection` and enters a loop that presents a menu to the user. Based on the user's choice, it calls the appropriate method on the `collection` object.

Let's take a closer look at this line:

```rust
io::stdout().flush().expect("Failed to flush stdout");
```

This line is crucial for ensuring that our prompt is immediately visible to the user. Here's a breakdown of what's happening:

1. `io::stdout()`: This returns a handle to the standard output stream.

2. `.flush()`: The `flush()` method is called on the stdout handle. In many systems, output is buffered by default, which means that text isn't immediately written to the screen but is stored in a buffer first. Flushing forces any buffered content to be written immediately.

3. `.expect("Failed to flush stdout")`: The `flush()` method returns a `Result`. We use `expect()` here to handle any potential errors. If `flush()` fails, the program will panic with the message "Failed to flush stdout".

Why is this important? In this case, we're using `print!()` instead of `println!()` to display the prompt. The `print!()` macro doesn't automatically flush the output buffer, while `println!()` does. Without flushing, the prompt might not appear until after the user has already started typing their input, leading to a confusing user experience.
### Behavior of print!() vs. println!()
* `print!()`: The `print!()` macro does **not** automatically flush the output buffer. This means that if you print something with `print!()` and don't explicitly flush, the output may stay in the buffer and not appear on the screen immediately.
* `println!()`: The `println!()` macro, on the other hand, does flush the output buffer automatically after printing. This is because `println!()` adds a newline character, which usually triggers a flush on most systems.

By explicitly flushing stdout, we ensure that the prompt appears on the screen before the program waits for user input, providing a smoother, more intuitive interaction.

For more information on input/output operations in Rust, you can refer to the [std::io module documentation](https://doc.rust-lang.org/std/io/index.html).

## Key Concepts and Rust Features Used

1. **Structs**: We use structs to define custom data types. Learn more about structs in the [Rust Book](https://doc.rust-lang.org/book/ch05-00-structs.html).

2. **Implementations**: The `impl` block allows us to define methods associated with our `Collection` struct. Read more about method syntax in the [Rust Book](https://doc.rust-lang.org/book/ch05-03-method-syntax.html).

3. **HashMap**: We use a `HashMap` to store our items. This provides efficient lookup and insertion operations. Learn more about HashMaps in the [Rust documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html).

4. **Pattern Matching**: We use `match` to handle different user choices. Explore pattern matching in the [Rust Book](https://doc.rust-lang.org/book/ch06-02-match.html).

5. **Error Handling**: We use `expect()` for simple error handling. For more advanced error handling techniques, check out the [error handling chapter](https://doc.rust-lang.org/book/ch09-00-error-handling.html) in the Rust Book.

6. **Ownership and Borrowing**: Notice the use of `&mut self` in some methods, which borrows the `Collection` mutably. Understanding ownership is crucial in Rust - learn more in the [ownership chapter](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) of the Rust Book.
