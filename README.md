# BPROG



## Description

A simple stack based interpreter written in Rust. It is a simple language that can be used to perform basic arithmetic operations, if statements, loops, functions and variable assignments.

## Features
- Integer and float arithmetic
- Boolean logic
- If statements
- Loops
- Functions
- Variable assignments
- Reading and printing values

## Usage
### Syntax
Because of the interpreter using words you have to make sure everything is separated by spaces. The interpreter will not accept anything that is not separated by spaces and will run them as one word. For a list it would look like this: `[ 1 2 3 ]`. same goes for blocks: `{ 1 2 3 }` and strings: `" hello world "`.

The intepreter only supports reverse polish notation (INFIX notation) for now. The syntax is as follows:
```
<expression> ::= <term> | <term> <operator> <expression>
<term> ::= <factor> | <factor> <operator> <term>
<factor> ::= <number> | <boolean> | <string> | <list> | <block> | <symbol>
<operator> ::= + | - | * | / | div | < | > | == | && | || | not
<number> ::= <integer> | <float>
<boolean> ::= true | false
<string> ::= " <character>* "
<list> ::= [ <expression>* ]
<block> ::= { <expression>* }
<symbol> ::= <identifier>
```

### Functions
As this can be looked a little like haskell we also have the following list functions:
```
words = splits a strong by spaces
head = returns the first element of a list
tail = returns the tail of a list
empty = returns true if the list is empty
length = returns the length of a list
cons = adds an element to the beginning of a list
append = appends two lists
foldl = folds a list from left to right
map = maps a list
each = executes a block for each element in a list
```

### Arithmetic
There are two types of arithmetic, integer and float. As it is strictly typed, you can only use integers for integer arithmetic and floats for float arithmetic. And they can not be mixed

For example if you wish to add two numbers, you can write  `1 2 +`.

### Boolean logic
There are two types of booleans, true and false. You can use them to perform boolean logic. For example if you want to check if a number is greater than another number you can write `1 2 >`. 

### if statements
If statements can be written as `<condition> if { <expression>* } { <expression>* }`. If the condition is true the first block will be executed, if the condition is false the second block will be executed, for example `10 5 5 == if { 10 + } { 100 + }` will print `20`. It also supports literals instead of a block, however it HAS to be only one symbol.

### Assignments
Assignments can be written as `symbol expression :=`. The symbol can be any identifier, and the expression can be any expression.

### Functions
Functions can be written as `symbol { <expression>* } fun `. They do not have to be defined before they are used, but they can only be used after they are defined.

### loops

Loops can either be written as `ammount times { <expression>* }` or `loop { <condition> } { <expression>* }`. Same as if statements, the condition can also be a code block or a literal.

### Stack operations
The stack can be manipulated using the following operations:
```
swap = swaps the top two elements on the stack
dup = duplicates the top element on the stack
pop = removes the top element from the stack
```

### Errors
Errors are handled by returning an error message. Specifically using the Result type. In REPL mode, the program will print the error message and continue running. In file mode, the program will stop and print the error message. Additionally if the stack does not have specifically 1 value on it, it will return an error in file mode.

### Running the program
By default the program will run in REPL mode, but you can also run it in file mode by passing the file name as an argument. REPL mode will print the stack after each expression is executed, while file will print out the last value on the stack IF it is has only one value on it.
To run the program, simply clone the repository and run the `main.rs` file like this:
```
cargo run -- file.txt
```
or to run it in REPL mode:
```
cargo run
```

You have to have the [Rust compiler](https://www.rust-lang.org/tools/install) installed to run the program. <br>

## Examples

### REPL
```
> 1 2 +
[3]

> 1 2 * 3 +
[5]

> 1 2 * 3 + 4 5 div -
[4]

> 1 2 * 3 + 4 5 6 div - swap dup + swap pop
[10]

> true if { println " true " } { println " false " }
true

> 1 loop { dup 4 > } { dup 1 + } [ ] 5 times { cons }
[[ 1 2 3 4 5 ]]

> 1 loop { dup 4 > } { dup 1 + } [ ] 5 times   cons  
[[ 1 2 3 4 5 ]]

> odd { dup 2 div swap 2 / == if false True } fun 
> 2 odd
[false]

> 3 odd
[true]

> age 20 := [ 10 age ] println
[[10 20]]

> read
This is a message from the user
> words println
[ " This " " is " " a " " message " " from " " the " " user " ]
