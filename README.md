# C-Lang - A scripting language without a lexer or anything really.

Since C-Lang has no lexer there are some issues.

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

To print a variable right beside some text you can use `printpv`
```
set Bob.
printpv Your name is:
// Outputs: Your name is: Bob
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

You can count to a specific number using `count`
```
count 14
```

To create empty lines in the output you can use `newl`
```
print Hello!

newl

print New line, New experience!
```

You can print out the date using `date`
```
date
```

#### Fun

Have you heard of that emmet snippet called Lorem?

Well there is a shorter version of the snippet in this language.

Just call: `lorem`

Chair.

```
chair
```

Legit binary conversion??
```
11
```

Saturn!
```
saturn
```


### Math

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

It is possible to generate random numbers using the `rand` keyword.
It will set the value of the variable to the randomly generated number!
```
rand 0 5
// Generates a random number between 0 and 5.
```

### Filesystem

C-Lang has a **very** basic way of interacting with the file system

Files are created file this:
```
nfile /path/to/file
```

Directories are created like:
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

You can remove directories and files like this::
```
//Files
delfile /path/to/file

// Directories
delfolder /path/to/directory
```

You can delete the script using
```
delscript
```
### Extensibility

You can run JS and Lua in C-Lang without using the `exec` command.

JS:
```
js file.js
```

Lua:
```
lua file.lua
```

### Audio

Wow this scripting language has alot of built-in features, it has no package system so I guess thats fair.

Play audio files using:
```
sound /path/to/file timelength
```

You can use variables:
```
soundv timelength
```

### Delays

You can create delays.

```
wait amount_of_seconds
print Delayed!
```