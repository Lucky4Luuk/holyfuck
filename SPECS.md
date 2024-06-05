# HolyFuck Specifications
These are still heavily subject to change! Nothing can be considered stable here.

## Stack
The stack is simply a list of values. Each value is a 8 bit unsigned integer. The stack does not have a specified size nor a size limit, and grows as needed. It can be interacted with using specific [operators](#operators).

## Memory
All memory is interfaced with through a memory pointer value. The size of this pointer is platform dependent, and can't directly be written to or read from. It can, however, be reset to 0, or read from the stack. See [operators](#operators). The memory pointer ALWAYS points to a 1 byte value, regardless of platform.

## Operators
All default Brainfuck operators still behave functionally the same, except for the input and output operators.

| Command | Description |
|---------|-------------|
| + | Increment value at the memory pointer [wrapping] |
| - | Decrement value at the memory pointer [wrapping] |
| > | Move memory pointer 1 byte to the right [wrapping] |
| < | Move memory pointer 1 byte to the left [wrapping] |
| [ | Jump past the matching ] if the byte at the memory pointer is 0 |
| ] | Jump back to the matching [ if the byte at the memory pointer is nonzero |
| . | Push the byte at the memory pointer to the stack |
| , | Pop a byte from the stack and write it to the current memory pointer |
| * | Pops N values from the stack. These values are turned into a single value (Little Endian, first byte popped = least significant byte) and used as the new active memory address. N is based on the current platform and the bits needed. A 32 bit platform will require N=4, but a 64 bit platform will require N=8! |
| ^ | Reset the current memory pointer to 0 |

TODO: Define what happens when popping from an empty stack.

## Functions
Functions are defined as follows:
```bf
:showme{
    +++[-]
}
```
Functions can be called as such:
```bf
:main{    
    @showme
}
```
Because the stack is shared throughout the entire program, there is no need for function arguments. Just make sure to pop the right values to the stack, and the function can read these and act upon them. Isn't that simple?

## Modules
Every good and modern language needs some sort of module/namespace system.
HolyFuck is no exception. Modules are declared simply by name, and imported with a special operator.
Example:
Let's pretend you have 2 files, `main.hf` and `coollib.hf`. Any functions defined in `coollib.hf` can be imported by doing the following:
```bf
pretend this is coollib_hf

This function will add 2 numbers and return the value back to the stack
Another approach could be to simply leave the value in memory and hope the caller is okay with the memory pointer being shifted
:add{
    ,>,<    Read our 2 input values from the stack
    [>+<-]  Add the 2 numbers
    >.<     Push the result to the stack
    [-]     Clean up the result from memory
}
```
```bf
pretend this is main_hf

#coollib
:main{
    +++.    Write 3 to memory and push it to the stack
    -.      Subtract 1 and push it to the stack as well
    --      Make sure the current memory cell is set to 0 so the function can use it
    @add    Run cool function
    ,       Lets pretend we print it here after popping it from the stack
}
```
This example shows how you might implement an addition function in a different module, import it and use it inside your main file!

File imports only pull the functions into your current namespace. It does NOT include the sourcecode of that file in your current file! You could import libraries as much as you want, and libraries can import other libraries. Circular dependencies are not supported.
To put files in a subfolder, you can just specify the path in your import.
Example: `math/add.hf` can be imported as `#math/add.hf`.
