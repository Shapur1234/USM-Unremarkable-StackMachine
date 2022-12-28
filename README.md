# USM - Unremarkable StackMachine
* USM is a simple [stack oriented](https://en.wikipedia.org/wiki/Stack-oriented_programming) programming language
* This repo includes an interpreter for it written in rust

## Language design
* Sourcecode is a text file ending with .usm
* Sourcecode is interpreted into a series of instructions by the interpreter
* The programm counter (a signed integer) stores which instruction will be executed next 
* When the programm registers value exceeds number of instruction, the programm will terminate
* Instructions are separated by a whitespace character, read from a file (see Examplpe programm)
* All programm data is stored on a single stack, every item on the stack is an unsigned integer (size depends on architecture)

### Language instructions:
| Instruction | Symbol | Meaning |
|-------------|--------|---------|
| NUM 3144    | 3144   | Pushes a value (any signed integer) onto the stack (Can be any number, doesn't have to be 3144) |
| POP         | !      | Pops the topmost value from stack and prints it to StdOut |
| CPY         | @      | Pops two topmost values from stack, copies the 2nd popped value that many times onto the stack |
| STACK_COUNT | $      | Puhses onto the stack the length of the stack |
| ADD         | +      | Pops two topmost values from stack, pushes result of adding them onto the stack |
| SUB         | -      | Pops two topmost values from stack, pushes result of subtracting them onto the stack |
| MUL         | *      | Pops two topmost values from stack, pushes result of multiplying them onto the stack |
| DIV         | /      | Pops two topmost values from stack, pushes result of dividing them onto the stack |
| MOD         | %      | Pops two topmost values from stack, pushes result of moduling them onto the stack |
| PUSH_PROGRAMM_COUNTER | <      | Pushes the programm register onto the stack |
| POP_PROGRAMM_COUNTER  | >      | Pops the topmost value from stack and makes it the new programm counter. Can be used as goto |
| STD_IN      | ?      | Read a value from StdIn, pushes the result onto the stack |

### Example programm
#### `? 2 * !`
* ? - Takes a number from StdIn
* 2 - Pushes 2 onto the stack
* \* - Multiplies the 2 numbers
* ! - Prints the result, popping it from the stack 

See [example_programms](/example_programms) for more examples

## Compiling the interpreter
* Install [rust](https://www.rust-lang.org/)
* Run `cargo r` to build and run the project

## Using the interpreter
* Precompiled binaries for (Arch) Linux and Windows can be found in [bin](/bin)
* Run `usm -h` for help
* Run `usm -f filepath` to interpret a file
* Run `usm -f filepath -d` to run in debug mode - intepret the code step by step
