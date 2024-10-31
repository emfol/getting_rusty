# Getting Rusty

Learning a new programming language is more than learning a new syntax.
Learning the new syntax is, of course, paramount but after writing your
first lines of code a few questions come up:

- How can I get my hands on the interpreter or compiler for my OS?
- How do I update my build environment?
- How do you get the computer to run those instructions?
- If the language is not an interpreted language, how do I build an
   executable out of those pitiful lines of code?
- How can I reuse code from other people?
- What is included in the standard library?
- What are the best libraries to do X and to do Y?
- How do I manage dependencies?
- How do I write unit tests?

I'm probably missing a few questions here but that is enough to get us
started. The bottom line is: Learning a new language is like migrating
to a new ecosystem.

The community behind the Rust Programming Language (which is my current
focus) seems to have put a lot of effort on responding those questions
upfront.

## The `rustup` Utility

The `rustup` utility is the answer for the first couple questions.
Yes, the community implemented a script which downloads, installs
and updates all the tools for your development environment.

Running the `rustup` utility is the first thing we need to do in order
to get Rust up and running on your machine.

The following one-line shell command will get the job done:

```shell
$ curl -fsSL https://sh.rustup.rs | sh
```

I'm usually very picky about blindly running a script directly
downloaded from the internet to perform changes on the file system
of my host machine. So, if you're like me, you'll create a container
before getting your hands dirty.

Let's do that. The following commands are going to create a container
based on the Debian 12 (Bookworm) image and add a mapping (bind mount)
of the current work directory to the `/app` directory inside the
container such that file changes on the host machine are visible inside
the container.

```shell
$ mkdir ~/path/to/rust/projects
$ cd ~/path/to/rust/projects
$ docker container create -it -v "$(pwd):/app" -w /app \
  --name getting_rusty --hostname getting_rusty \
  debian:bookworm /usr/bin/env bash
$ docker container start -ai getting_rusty
```

This repository has a script to facilitate running the commands above.
If you prefer to use it instead of those commands, just type:

```shell
$ ./init.sh
```

After attaching to the container, you can run the first shell command
listed above which will install the `rustup` utility.

You can use this utility the manage version of your rust tool chain.
For example, you can use the following command to update to the latest
version of the Rust compiler and package manager:

```shell
$ rustup update
```

Or, if you have given up on Rust, you can uninstall it with:

```shell
$ rustup self uninstall
```

Along with the `rustup` utility other two important pieces of software
are also installed:

- `rustc`: The Rust compiler.
- `cargo`: The build system and package manager for Rust.

Those tools will be discussed next.

## The Rust Compiler: `rustc`

This is the most important piece of software downloaded by the `rustup`
utility. Rust is a compiled language which means that after writing any
code, you need to compile it before it can be run by the operating
system. The `rustc` utility is responsible for that. You can check if
it was correctly installed with:

```shell
$ rustc --version
```

You can check if you can compile code with a very simple hello-world
application. Create a file called "hello_world.rs" like in the example
below:

> Files containing Rust source code must have the `*.rs` extension.

```rust
fn main() {
  println!("Hello, World!");
}
```

Then run the compiler with:

```shell
$ rustc hello_world.rs
```

If you check the contents of your current work directory, you should
see a file named `hello_world` (if you're on a UNIX-like operating
system). That file is the resulting executable which can be run with:

```shell
$ ./hello_world
```

> If your compiler complains about a missing linker, it means you do
> not have the basic development tools for your platform installed.
> For the debian container mentioned above, you  can install these
> tools with the following command: `apt-get install build-essential`.

## Cargo: The Build System and Package Manager for Rust

For compiling a handful of files, all you need is the `rustc` utility.
For larger projects though, with hundreds of files and lots of
dependencies, you need something else: Cargo.

Cargo is the official build system and package manager of the Rust
programming language. It can be used to initialize new projects,
download and update dependencies.

A new project can be created with:

```shell
$ cargo new my_project
```

The newly created project can be built with:

```shell
$ cd my_project
$ cargo build
```

This will create an executable with the same name of the project in the
folder `target/debug` which can be run with:

```shell
$ ./target/debug/my_project
```

Notice that a debug version is built by default. When you're ready to
build a release version, you can build with the `--release` parameter:

```shell
$ cargo build --release
```

The release version of the new executable will be found at directory
`target/release` with the same name of the project and it can be run
with:

```shell
$ ./target/release/my_project
```

Another nice thing about the Cargo utility is that you can directly run your project with a single command instead of repetitively running the build + run commands. This can be achieved with:

```shell
$ cargo run
```

## The Rust Programming Language

After a brief introduction of the utilities available in the development
environment of the Rust Programming Language, it's time to dive into the
language itself.

### Introduction

Rust is a general-purpose programming language with emphasis in
performance, concurrency, type and memory safety without relying on
garbage collection. Rust programs are compiled to machine code which are
directly executed by the processor of the target platform making it a
popular choice for systems programming.

Being a systems-level programming language means that Rust deals with
the same sort of objects that most computers do: characters, numbers
and references (in spite of the rich set of high-level data structures
provided by the standard library). The type and size of variables must
be declared before use and a good understanding of the differences
between stack and heap allocations is helpful.

### Data Types

Rust is a statically typed language which means that every value in the
program must be of a certain data type known by the compiler at compile
time. A set of basic data types is provided by the language which can
then be combined into compound data types to form complex data
structures.

### Basic Data Types

Rust has four primary scalar types:

- Integers:
  - Signed:
    - i8, i16, i32, i64, i128, isize;
  - Unsigned:
    - u8, u16, u32, u64, u128, usize;
- Floating-Point Numbers:
  - f32, f64;
- Booleans:
  - true, false;
- Characters:
  - Single-character literals specified in single quotes (e.g., '🦀')
    internally represented as 32-bit Unicode Code Point;

### Compound Types

Rust provides the following builtin compound types which always have
fixed sizes and are allocated on the stack:

#### Tuple

```rust
let tuple: (i32, f32, char) = (2, 3.14, '🦀');
```

#### Array

```rust
let array: [i32; 4] = [1, 2, 3, 4];
```

### Statements vs Expressions

### Functions

> The `main` function is one of the most important functions in the
> language since it is the entry point of many programs.
