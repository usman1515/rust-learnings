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
- Alternative name that can be assigned to an exsting type.

## 38. Compiler Directives
- Annotation that tells the compiler how to parse the source code.
- Write a directve above a line to apply it for that line.
- Write a directve above a function to apply it for that function.

