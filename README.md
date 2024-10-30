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
$ docker container create -it -v "$(pwd):/app" --name getting-rusty \
  --hostname getting-rusty debian:bookworm /usr/bin/env bash
$ docker container start -ai getting-rusty
```

This repository has a script to facilitate running the commands above.
If you prefer to use it instead of those commands, just type:

```shell
$ ./init.sh
```
