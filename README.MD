![venti](venti.png)

# Venti
venti is an simple programming languange that made with rust, i made it because i love venti lol

## TODO
- [x] Printing and Variable Creation
- [x] Looping
- [x] Function Definitions
- [x] Error Handling
- [x] Compiler Integration with LLVM
- [x] Data Structures(e.g., array, lists)
- [x] Control Flow Statements(e.g., conditionals)
- [x] Basic Math Operation
- [x] Type System Implementation
- [x] Memory Management
- [x] Foreign Function Interface (FFI)
- [ ] Concurrency and Parallelism Features
- [ ] Standard Library Functions
- [ ] Documentation and Examples

# Example

## Basic Syntax
```py
# venti nama_variabel = data_type;
venti contohString = "I Love Venti";
venti contohInteger = 69;
venti iLoveVenti = true;

printventi(contohString);
printventi(contohInteger);
printventi(iLoveVenti);

venti myArray = ["i", "love", "venti"];
printventi(myArray);

venti myNestedArray = [[1, 2], [3, 4], [5, 6]];
printventi(myNestedArray);
```

## Type Annotations
```py
venti x: int = 5;
venti y: float = 3.14;
venti z: bool = true;

funcVenti add(a: int, b: int) {
    venti result: int = a + b;
    print(result);
}

add(5, 10);
```

## Basic Math Operations
```py
venti operasi = (4 + 4 * 2)/2;
print(operasi);

venti faktorial = factorial_venti(5);
print(faktorial);
```

## Looping (For Loop & While Loop)
```py
for_venti i in 100 {
    venti myString = "I Love Venti 100x";
    print(myString);
}

while_venti(true) {
    venti myString = "I Love Venti";
    print(myString);
}
```

## Function
```py
funcVenti myFunction(a, b) {
    venti result = a + b;
    printventi(result);
}

venti x = 10;
venti y = 20;
myFunction(x, y);
```

## Conditionals
```py
venti x = 1;
if_venti(x > 1) {
    venti result = "i love venti";
    printventi(result);
} else_venti {
    venti result = "i really love venti";
    printventi(result);
}
```

## Variable & Functions With Memory Management
```py
venti str: string = "I Love Venti!";
print(str);

venti arr: array = [1, 2, 3];
print(arr);

funcVenti append(arr: array, val: int) {
    arr.push(val);
}

append(arr, 4);
print(arr);
```

# Installation
1. Clone the repo and `cd` into it
2. Run `cargo build` to build it from source
3. Then you execute the compiled binary using the command below:

```bash
target/debug/venti -h
```

# Compiling 
Write some valid venti code like the following:
```py
printventi("i love venti");
```

Now compile it using
```bash
lli ~/output.ll
```

**NOTE**: Venti is currently on development mode. At the moment, it only supports basic features like printing, making variable, data types, looping, etc. I plan to add more features. And venti doesn't generate executable binaries yet, it generates LLVM IR in a file called `output.ll` in the same folder as the source file that you  can run with the `lli` command that comes with your LLVM installation.






