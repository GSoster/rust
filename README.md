# rust
Repo for Rust practice and learning


## Environment Setup

To use Visual Studio Code, follow instructions [here](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Resources

- [ ] [The Rust Book](https://doc.rust-lang.org/book/)
- [ ] [Rustlings - 🦀 Small exercises to get you used to reading and writing Rust code!](https://github.com/rust-lang/rustlings)
- [ ] [Exercism Rus Track](https://exercism.org/tracks/rust/)
- [ ] [Ultimate Rust crash course](udemy.com/course/ultimate-rust-crash-course/)
- [Path with many exercises](https://github.com/jondot/rust-how-do-i-start)

## Annotations

### Running Rust, File structure and naming

Rust files always end with the *.rs* extension. If you’re using more than one word in your filename, the convention is to use an underscore to separate them. For example, use *hello_world.rs* rather than helloworld.rs.

To compile a .rs file into executable execute `rustc filename`: `rustc main.rs`

**using Cargo** 

We can create a project using cargo new.  
We can build a project using cargo build.  
We can build and run a project in one step using cargo run.  
We can build a project without producing a binary to check for errors using cargo check.  
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.  
When your project is finally ready for release, you can use cargo build --release to compile it with optimizations.  

### Variables

`let apples = 5;` This line creates a new variable named apples and binds it to the value 5. In Rust, variables are immutable by default, meaning once we give the variable a value, the value won't change.  
To make a variable mutable, we add *mut* before the variable name: 
```rs
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

#### Associated Functions

`let mut guess = String::new();` The :: syntax in the `::new` line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string.  
Associated Functions = Static functions in C#

#### References

`io::stdin().read_line(&mut guess)` he & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. Like variables, references are *immutable by default*. Hence, you need to write **&mut** guess rather than &guess to make it mutable.

#### Shadowing
