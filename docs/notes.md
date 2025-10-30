# Rust NOTES

## 15. Create rust projects with `cargo`
- create new project: `cargo new <project_name>`
- rust projects are often called `crates` or `packages`.
- 2 types of rust crates
    - binary crates: standalone rust application whose purpose is to be run by itself alone.
    - library crates: meant to be incorporated into another rust project. not standalone.
- file extension `.rs`.

### project hierarchy
```
<project_name>
├── src/            -- all project code files here
│   └── main.rs
├── target/         -- executables are generated here
└── Cargo.toml      -- stores metadata and info about the rust project.
└── Cargo.lock      -- automatically generated and updated. keep in version control.
```

## 16. Hello World
- Everything starts with a function and is inside one.
- Every rust program must have a `fn main() {}` at all costs. Like C.
- `println!()` is a macro.
    - The `!` reminds us its a macro.
    - From rust perspective there is no difference between alphabetical characters and symbols.
- Your executable is generated here: `<project_name>/target/debug/<project_exec_name>

### Running your code
```bash
cd <sample_prj>                     # goto prj dir
cargo clean --package <sample_prj>  # clean all bin
cargo check --package <sample_prj>  # check for errors but dont build
cargo build --package <sample_prj>  # compile/build
cargo run --package <sample_prj> --bin <sample_prj>     # execute bin
```

- generate executable
```
rustc ./crate_00_hello_world/src/main.rs
./main
```

## 17. Compilation
- Generated executable is architecture and platform specific.
- Just like C, Linux executable wont run on Mac.

## 18. Formatting with rustfmt and cargo fmt.
- Standardized formatting tool is builtin the language.
- format a single file: `rustfmt ./<your_file>.rs`
- Format entire project: `cargo fmt`

## 20. `cargo build` command
- by default `cargo build` runs in debug mode.
- Generate final version in "release mode". Takes longer to compile but executable is more optimized.

## 22. Comments
- line comments: `//`
- block comments: `/* */`
- just like C

## 28. Intro to variables
- Create a variable: `let apples = 50;`
- Interpolating is placing/printing a certain value inside a string.

## 32. Immutable and Mutable variables
-  Variables in rust are immutable by default.
- *Immutable* means its value cant be changed.
- Set them as mutable so that they can be updated further down the road.
- However we cant change the type.

- [rust error code index](https://doc.rust-lang.org/error_codes/error-index.html)

## 34. Variable shadowing
- Redeclaring a variable from scratch.
    - The original variable is "replaced" by a new one.

## 35. Scopes
- Scope is a boundary or a region of code in which a name is valid.
- Block: area of code that represents a scope.

## 36. Constants
- Variables are limited to a function scope.
- Constants can be declared at any scope and can be used throughout the file.
- Can be declared outside function in the open.
- `const TAX_RATE: f64 = 3.45;`
- For constants type must be inferred at all costs. Variables tyep are inferred at run time.

## 37. Type Aliases
- Alternative name that can be assigned to an existing type.

## 38. Compiler Directives
- Annotation that tells the compiler how to parse the source code.
- Write a directive above a line to apply it for that line.
- Write a directive above a function to apply it for that function.

## 43. Introduction to data types
- A scalar type holds only 1 value.
    - Rust has 4 scalar type.
- Rust has signed and unsigned integers.
- `i32` mean signed 32 bits.
- `u32` mean unsigned 32 bits.
- Can store 8, 16, 32, 64, 128 bits data in variables.
- `i32` and `f64` are defaults data types for integers and floats.

## 46. `usize` and `isize`
- Use the type aliases `usize` and `isize` when declaring integers.
- These types assign width to a variable based on the architecture it will run.
- So on a 32 bit machine they will be 32 bit and 64 bit on a 64 bit machine.

## 47. Strings and Raw Strings
- Strings inside print function are "string literals".
    - Their value is known at compile time.
- Add a `\` before some special char to print it as is e.g. `\\n`.
- In a raw strings every char is printed as is. Ideal for file paths.

## 48. Methods
- A function that lives on a value.
    - Its an action we ask the value to execute.
- A basic set of functions that can be run a variable.
- Methods can also accept arguments.

## 49. Floats
- Default is `f64`.
- `f64` has 15-17 digit precision.
- `f32` has 5-7 digit precision.

## 50. Float specifiers
- A float specifier customizes the printed representation of the interpolated value.

## 52. Math Operations
- Rust does floor division on integers.
    - When you divide 2 integers you will get a integer.

## 54. Boolans
- True or False. Takes 1 byte.

## 59. The Char type
-  Take 1 byte.
- Unicode is a computing standard for the representation of textand most of the world's writing
systems. Supports various symbols and emojis as well.
- **UTF** - Unicode Transformation Formation.
    - Has svereal variants.
    - UTF-8, UTF-16, UTF-32
- Char takes 4 bytes inorder to support all UTF characters.

## 60. Arrays
- An array is a fixed size collection of a homogenous datatype.

## 62. Traits
- A trait is a contract that requires that a type support one or more methods.
- Traits establish consistency between types.
    - Methods that represent the same behavior can have the same name across different types.
    - A type **implements** a trait.
    - A type can chhose to implement multiple traits.
- **Display trait**.
    - Built in.
    - Using `{}` in strings.
- Some types can't be displayed.
- **Debug trait**
    - Goal is to format a given type into a programmer-facing string for debugging purposes.
    - `{:?}` or `{<some-variable-array>:?}` is the default debug trait.
    - `{:#?}` or `{<some-variable-array>:#?}` is the pretty print debug trait.

## 64. `dbg!` macro
- Prints and returns the value of an expression for quick and easy debugging.
- Basically the basic print statement on steroids for the developer.

## 65. Tuples
- Collection type.
- Support values of different types.

## 66. Ranges and Range Intervals
- Ranges are exclusive by default.
    - Goes up to max number but doesnt include it.


