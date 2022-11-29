# Rust, The Programming Language

## What is Rust?
Rust is a multi-paradigm, strongly typed, general-purpose programming language focused on providing runtime 
safety via strict language features and compilers. Unlike many other languages, Rust does not use a garbage
collector or reference counting to manage memory and uses a technology called borrow checking to manage 
the lifetime of memory during the compilation of the program.

The initial debut was in January 2014 however development started at Mozilla, the developers of Firefox, 
back in 2008. While influenced heavily by existing programming languages the team at Mozilla were determined
to provide a low level systems language that was easy to write safe code in. This is in contrast to languages
such as C or C++ where memory safety is a constant issue making up a good portion of security issues we see
in modern software.

The foundation Rust is built upon is a technology called LLVM which provides an intermediary target for code 
to be compiled to which then can be compiled and optimized for specific targets. This allows Rust to adopt 
many of the speed improvements and general compatibility from the LLVM ecosystem giving it another head start
in the space.

## Why and where should I use Rust?
Rust is now widely adopted and has an expansive ecosystem being created around it with support from major 
companies such as Discord, Meta, and Microsoft. With a robust package manager code named Cargo, a rich type
system, and the aforementioned inherent safety and speed of the language.

Since Rust is generally aimed at being a low level systems language its main purpose is to be at the backbone
of microservice architectures, web servers, and embedded systems. One good example of this is in its use on 
high-performance blockchains such as Solana which it is used in the backend node software and by the majority 
of smart contracts.

The world doesn't stop at embedded systems and blockchains though many cross-platform apps are being built 
using an emerging framework code-named [Tauri](https://tauri.app/). Tauri provides a way to write a desktop
apps backend on rust and then communicate to any UI/UX or web framework through a rich messaging system!

There are many more emerging high-performance frameworks though and the world is using Rust more and more 
every day.

### Syntax and Language Features
 - Primitive Types
 - Integer (Signed and Unsigned)
 - Floating Point
 - Boolean
 - Pointer Sized Integers
 - Characters
 - Strings
 - Arrays
 - Tuples
 - References

More in depth information on the expansive primitive types provided can be found 
[here](https://doc.rust-lang.org/book/ch03-02-data-types.html).

### Additional Types
Unlike other low level system languages rust provides additional types to aid in development as follows.
 - Box - A value stored inside the heap
 - String - Instead of static or mutable these strings are dynamic
 - Vec<T> - A dynamic array implementation similar to Java’s ArrayList.
 - Option<T> - An option type used heavily in the language, more on this later.
 - Result<T, E> - Error handling through option types.

## Ownership, Moving, and Borrowing, are arguably the most important feature!
Ownership is the bread and butter of Rust, to move away from the traditional standard of manual memory 
allocation or garbage collectors, introduce ownership and borrowing.

Variables allocated to the heap are alive for their scope and will be considered dead and deallocated 
outside their scope. A great example from the Rust Documentation is as follows.

```rs
{
let s = String::from("hello"); // s is valid from this point forward and owned by s

// do stuff with s
}   // this scope is now over, and s is no longer valid	
```
This demonstrates something called ownership, the variable s and its associated data was valid in the
scope of s, but as soon as the scope ended the data at the pointer of s was freed by the runtime.

Since Rust only allows one variable to own a set of data at a time we need to move onto the properties
of moving data. In other languages, this is known as shallow copying and by other names but in Rust 
it behaves differently.
```rs
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // ERROR!: s1 is no longer valid as the data is owned by s2!
```
The variable s1 is invalid by the time the print macro is run, this is due to an operation called moving. 
Since s2 was assigned ownership of the data on the heap, s1 can no longer access it! This is enforced on 
a compiler level.

One side effect of this that may seem to contradict these rules are when working with stack-only data 
that has a length known to the runtime.
```rs
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```
This code works! That is because the size of x is known at runtime, in this case most likely a u8, so 
the runtime can quickly make a copy of the data to y, still enforcing that data can only be owned by 
one variable at a time.

Note that function calls work intuitively and pass ownership in and out where applicable to maintain 
these rules.

## Structs, Macros, Pattern Matching, and more
We will demonstrate a few of these below however, Rust is a very feature rich language we will not be 
able to cover everything that makes it such a great language to work in. However, the developers have
done an amazing job at putting together documentation that goes more in depth on these different language
features which can be found [here](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html).

## Setup Environment
Instructions below will be for linux based machines, find relevant sources in Quick Links to assist on
alternative platforms.

To start off we need to install the rust compiler toolchain via Rust-up, make sure to set up path variables
correctly with onscreen instructions.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Since Rust is in constant development we will want to use the nightly branch to access some cool features
locked behind feature flags. Run the following commands to download and set the latest nightly build as 
the default toolchain.
```
rustup toolchain install nightly
rustup default nightly
```

To confirm the toolchain was installed correctly execute the following and ensure a valid version number 
is returned
```
rustc --version
```
Compile Your First Program
It is recommended to use an IDE such as VSCode and or any Jetbrains IDE with the relevant Rust language 
feature packs enabled to assist in development.

To create a new project using Cargo, type `cargo new` and follow the on screen prompts.

Change directories into the folder which is the name of the crate you choose. To execute the example program 
initialized in `main.rs`, type `cargo run`.

## Managing Packages Using Cargo
Cargo is the built tool created for the ecosystem. Any set of code, even a single main.rs file, is considered 
a crate by the cargo build tool. To add additional packages such as `thiserror`, we will need to manually
add the dependency to the cargo.toml file located at the root directory of our project.

The `Cargo.toml` file will look something like this,
```
… (some code above)
[dependencies]
```
To add a new dependency we will insert it below as follows,
```
… (some code above)
[dependencies]
thiserror = "1.0.37"
```
Finally, we will run either `cargo build` or `cargo update` to fetch the crate and it will be available for 
use by our runtime.

## Rust Program Examples
### Hello, World!
```rs
// Declare a function using the “fn” keyword
fn main() {
    println!("Hello, world!"); // “println!” Is a macro, macros are indicated by an exclamation mark
}
```

Let's add a bit of functionality, maybe some arithmetic as well.

### Simple Add Function
```rs
fn main() {
    // Create some vars
    let x = 10;
    let y = 10;
    // Call our function and store its result
    let result = add_two_numbers(x, y);
    // Print the result using the formatting capabilities of println!
    // {} are replaced by tokens passed into the macro.
    println!("This is the result!: {}", result);
}

// Take two i32's and add them together to result in an i32!
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b // Return statement not needed as it is the last line in the function call
}
```

### Snake Game
To show off a bit more about rust and its features in action we have made a snake game.
We will be unable to include its entirety inside this site and will provide snippets as we walk through a 
few language features we used.

If you would like to check out the git repository that can be found [here](https://github.com/Chasewhip8/snake).

The dependencies used are listed below with a description of what they do
 - pancurses = "0.17"  - Provides an api to interface with terminals cross-platform.
 - rand = "0.8.5" - Provides random number generation functions.

#### Matching
Data can be matched using Rust's powerful match keyword to deconstruct enums and more. In this case we use it to
match each case of our Control enum. `_` represents the default case to fall back to as Rust **requires** all code paths
to be exhausted fully.
```rs
    fn get_next_point(board: &Self) -> Point {
        let head = board.snake.last().expect("Invalid Game State, Snake is empty");
        return match board.move_direction {
            Control::UP => Point::from(head.x, head.y - 1),
            Control::DOWN => Point::from(head.x, head.y + 1),
            Control::LEFT => Point::from(head.x - 1, head.y),
            _ => Point::from(head.x + 1, head.y)
        }
    }
```

#### Struct and Impls
Structs are similar to C and C++ where a data type can be defined and modified. Implementations are provided externally
but have access to the locally scoped instance referenced by `Self`. In this case we need to use a derivation of PartialEq
and Clone to allow rust to auto generate code to compare and clone this struct, this can only be done when the 
compiler knows how to do these operations on all data types present.
```rs
#[derive(PartialEq, Clone)]
pub struct Point {
    x: u8,
    y: u8
}

impl Point {
    pub fn from(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u8 {
        self.x
    }

    pub fn y(&self) -> u8 {
        self.y
    }
}
```

#### Results, Options and if let
Rust relies heavily on its extensive type system which brings us Options and Results. As we can see here an
option type is used to indicate sometimes a Point can be assigned to `board.fruit`, other times it will be 
`None`. Result<T,E> is another enum which allows us to hold a value and a possible error. Both of those types can
be used in a similar manner with if let to define a variable and enter a scope conditionally.
```rs
    pub fn check_collision_food(board: &mut Self){
        if let Some(fruit) = &board.fruit {
            let last_body = board.snake.last().cloned().expect("Invalid State, Snake is empty");
            if *fruit == last_body {
                board.state = FruitCollected;
                board.fruit = None;
            }
        }
    }
```

## External Links
Here is a list of external links that provided useful information and examples used in this writeup.
 - Rust Installer: https://rustup.rs/
 - Cargo References: https://doc.rust-lang.org/stable/cargo/reference/index.html
 - Rust References: https://doc.rust-lang.org/stable/rustc/index.html
 - Standard Library Reference: https://doc.rust-lang.org/stable/std/index.html
 - Rust Guide: https://doc.rust-lang.org/book/title-page.html