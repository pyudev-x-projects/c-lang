# C-Lang - A scripting language without a lexer or anything really.

Since C-Lang has no lexer there are some issues.

You cannot have empty lines or else the Rust compiler will complain: "index out of bounds: the len is 0 but the index is 0"

## Installation

Install by running:
```sh
cargo install c-lang
```
OR
```sh
cargo install --git "https://github.com/pyudev-x-projects/c-lang"
```

## Learn C-Lang

### Hello World

```
print Hello World!
```

There are no string data types in C-Lang
Infact, there are no data types at all!

### Comments

Comments are created using `//` like in a lot other programming languages.
```
// I am ignored by the interpretter!
```

### Variables

You are limited to only 1 variable in C-Lang 

You can set that variable using the `set` keyword

```
set 45
```

To print a variable you can use the `printv` keyword

```
printv
```

To set a variable with input you can use the `input` keyword

```
print Enter your name please!
input
print Your name is:
printv
```

There are much more keywords to use variables but we will explain those later.

### Utility keywords

#### Console

You can change the color of the output with a simple keyword!


```
setclr red
print I am red!
setclr blue
print I am blue!
```

List of colors are: red, blue, green, yellow, magenta, cyan, white, black

You can clear the output using `clear`
```
print I am not visible!
clear
print Terminal cleared.
```

#### Fun

Have you heard of that emmet snippet called Lorem?

Well there is a shorter version of the snippet in this language.

Just call: `lorem`


## Math

For math there is a keyword called `printc`
Which stands for: "print calculation"

You can use it like this

```
// Addition
printc 4 + 3
// Subtraction
printc 4 - 3
// Multiplication
printc 4 * 3
// Division
printc 4 / 3
```

You must include a whitespace between each argument or else it will result in an error!

## Filesystem

C-Lang has a **very** basic way of interacting with the file system

You can really only create files and directory. You cant even delete them (For now)

Files are created file this:
```
nfile /path/to/file
```

Directorys are created like:
```
ndir /path/to/directory
```

You can also use variables

```
// Creates a directory on behalf of the variable
ndirv
// Creates a file on behalf of the variable
nfilev
```