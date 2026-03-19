# i Language Complete Reference Guide

## Table of Contents
1. [Language Philosophy](#language-philosophy)
2. [Core Concepts](#core-concepts)
3. [Data Types](#data-types)
4. [Built-in Functions](#built-in-functions)
5. [Syntax Reference](#syntax-reference)
6. [Code Examples](#code-examples)
7. [Implementation Details](#implementation-details)

---

## Language Philosophy

The i programming language is inspired by Zen philosophy and embraces minimalism. Every aspect of the language is designed to be:

- **Simple**: Complex problems solved with elegant solutions
- **Efficient**: Maximum functionality with minimum resources
- **Clear**: Code that reads like natural thoughts
- **Flow**: Programming as a meditative practice

### Core Zen Values

The language is built around three fundamental concepts:

```lisp
zen    ;; The state of emptiness, represented as 0
flow   ;; The state of being, represented as true  
void   ;; The state of nothingness, represented as nil
```

---

## Core Concepts

### Values and Types

The i language uses a simple type system with six core value types:

#### 1. Nil (Void)
```lisp
void        ;; Represents nothingness, absence of value
```
- **Purpose**: Represents the absence of any value
- **Usage**: Returned by functions that don't produce meaningful output
- **Truthiness**: Always evaluates to false

#### 2. Numbers
```lisp
42          ;; Integer
3.14        ;; Floating point
-7          ;; Negative number
0           ;; Zero (also accessible as 'zen')
```
- **Purpose**: Mathematical calculations and numeric data
- **Type**: f64 (64-bit floating point)
- **Truthiness**: Zero is false, any non-zero is true

#### 3. Text (Strings)
```lisp
"Hello"     ;; Text string
"i language" ;; Multi-word text
""          ;; Empty string
```
- **Purpose**: Text data and messages
- **Syntax**: Double-quoted strings
- **Truthiness**: Empty string is false, non-empty is true

#### 4. Booleans
```lisp
flow        ;; True (built-in constant)
true        ;; Boolean true value
false       ;; Boolean false value
```
- **Purpose**: Logical operations and conditions
- **Built-in**: `flow` represents the true state

#### 5. Lists
```lisp
[1 2 3]     ;; List of numbers
["a" "b"]   ;; List of text
[1 "two" 3] ;; Mixed list
```
- **Purpose**: Collections of ordered values
- **Syntax**: Square bracket notation
- **Truthiness**: Empty list is false, non-empty is true

#### 6. Maps (Dictionaries)
```lisp
{key: value}      ;; Key-value pair
{name: "i" version: 1}  ;; Multiple pairs
```
- **Purpose**: Named collections of values
- **Syntax**: Curly brace notation with colon separators
- **Truthiness**: Empty map is false, non-empty is true

---

## Built-in Functions

### Input/Output Functions

#### `echo`
```lisp
(echo "Hello World")        ;; Prints: Hello World
(echo 42)                   ;; Prints: 42
(echo (add 1 2))            ;; Prints: 3
```
- **Purpose**: Output values to the console
- **Parameters**: 1 or more values
- **Returns**: `void`
- **Behavior**: Converts values to readable format and prints

### Arithmetic Functions

#### `add` (Addition)
```lisp
(add 1 2)                   ;; Returns: 3
(add 1 2 3 4)              ;; Returns: 10
(add 3.5 2.1)              ;; Returns: 5.6
```
- **Purpose**: Sum multiple numbers
- **Parameters**: 2 or more numeric values
- **Returns**: Number (sum of all inputs)
- **Error**: Requires at least 2 arguments, all must be numbers

#### `sub` (Subtraction)
```lisp
(sub 10 3)                  ;; Returns: 7
(sub 20 5 2)               ;; Returns: 13 (20 - 5 - 2)
```
- **Purpose**: Subtract numbers from left to right
- **Parameters**: 2 or more numeric values
- **Returns**: Number (result of sequential subtraction)
- **Behavior**: `result = arg1 - arg2 - arg3 - ...`

#### `mul` (Multiplication)
```lisp
(mul 4 5)                   ;; Returns: 20
(mul 2 3 4)                ;; Returns: 24
(mul 1.5 2)                ;; Returns: 3.0
```
- **Purpose**: Multiply multiple numbers
- **Parameters**: 2 or more numeric values
- **Returns**: Number (product of all inputs)

#### `div` (Division)
```lisp
(div 20 4)                 ;; Returns: 5
(div 100 5 2)              ;; Returns: 10 (100 / 5 / 2)
```
- **Purpose**: Divide numbers from left to right
- **Parameters**: 2 or more numeric values
- **Returns**: Number (result of sequential division)
- **Behavior**: `result = arg1 / arg2 / arg3 / ...`

### Control Flow Functions

#### `if` (Conditional)
```lisp
(if flow "yes" "no")        ;; Returns: "yes"
(if (sub 5 5) "true" "false") ;; Returns: "false" (0 is false)
```
- **Purpose**: Conditional selection based on truthiness
- **Parameters**: 3 values (condition, then-branch, else-branch)
- **Returns**: Value from selected branch
- **Logic**: If condition is truthy, return second value, else return third

---

## Syntax Reference

### Expression Syntax

The i language uses a simple, consistent syntax based on S-expressions:

#### Function Calls
```lisp
(function_name arg1 arg2 arg3)
```
- **Format**: Parentheses, function name, then arguments
- **Spacing**: Spaces separate all elements
- **Nesting**: Functions can be nested as arguments

#### Literals
```lisp
42          ;; Number literal
"text"      ;; Text literal
flow        ;; Identifier/constant
zen         ;; Built-in constant
void        ;; Built-in constant
```

#### Lists and Maps
```lisp
[1 2 3]                    ;; List
{key: value}              ;; Map
[1 "two" (add 1 2)]       ;; List with mixed types
```

### Tokenization Rules

The language tokenizer follows these rules:

1. **Whitespace**: Spaces, tabs, newlines are ignored (except as separators)
2. **Parentheses**: `(` and `)` denote function calls
3. **Quotes**: `"` starts and ends text strings
4. **Numbers**: Digits, optional decimal point, optional leading minus
5. **Identifiers**: Alphanumeric characters and underscores

### Parser Behavior

The parser uses recursive descent with these rules:

1. **Literals**: Numbers, text, and built-in constants parse directly
2. **Variables**: Any identifier not recognized as a built-in
3. **Function Calls**: Must start with identifier after opening parenthesis
4. **Argument Parsing**: Continues until matching closing parenthesis

---

## Code Examples

### Hello World
```lisp
;; Basic output
(echo "Hello, World!")
```
**Explanation**: 
- `echo` is the output function
- `"Hello, World!"` is a text literal
- The entire expression prints the greeting

### Zen Philosophy Demo
```lisp
;; Demonstrate core Zen values
(echo "Zen is:")
(echo zen)                    ;; Prints: 0

(echo "Flow is:")
(echo flow)                   ;; Prints: true

(echo "Void is:")
(echo void)                   ;; Prints: void
```
**Explanation**:
- Shows the three fundamental constants
- Each value is printed using `echo`
- Demonstrates the language's philosophical foundation

### Arithmetic Calculator
```lisp
;; Basic arithmetic operations
(echo "Addition:")
(echo (add 5 3))             ;; Prints: 8

(echo "Multiplication:")
(echo (mul 4 6))             ;; Prints: 24

(echo "Complex expression:")
(echo (mul (add 2 3) 4))    ;; Prints: 20
```
**Explanation**:
- `add` and `mul` perform arithmetic
- Functions can be nested: `(add 2 3)` is calculated first
- Result: `(add 2 3)` → 5, then `(mul 5 4)` → 20

### Conditional Logic
```lisp
;; Conditional example
(echo "Is flow true?")
(echo (if flow "Yes, flow is true" "No, flow is false"))

(echo "Is zero true?")
(echo (if zen "Yes, zero is true" "No, zero is false"))
```
**Explanation**:
- `if` evaluates the first argument's truthiness
- `flow` is true, so returns first branch
- `zen` (0) is false, so returns second branch

### List Operations
```lisp
;; Working with lists
(echo [1 2 3])              ;; Prints: [1 2 3]
(echo ["a" "b" "c"])        ;; Prints: ["a" "b" "c"]
(echo (add 1 2 3))          ;; Compare with list syntax
```
**Explanation**:
- Lists use square bracket notation
- Lists are displayed with their contents
- Different from function calls despite similar appearance

---

## Implementation Details

### Core Data Structures

#### Value Enum
```rust
pub enum Value {
    Nil,                           // void/absence
    Number(f64),                   // 64-bit floating point
    Text(String),                  // String data
    Boolean(bool),                 // True/false values
    List(Vec<Value>),             // Ordered collection
    Map(hashbrown::HashMap<String, Value>), // Key-value pairs
}
```

#### Interpreter Structure
```rust
pub struct Interpreter {
    globals: hashbrown::HashMap<String, Value>,  // Global variables
    stack: Vec<Value>,                          // Execution stack
}
```

### Execution Pipeline

1. **Tokenization**: Source text → Token vector
2. **Parsing**: Token vector → Abstract Syntax Tree (AST)
3. **Evaluation**: AST → Result value

### Truthiness Logic

```rust
impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,                    // void is false
            Value::Boolean(b) => *b,                // Use boolean value
            Value::Number(n) => *n != 0.0,          // Zero is false
            Value::Text(s) => !s.is_empty(),        // Empty string is false
            Value::List(l) => !l.is_empty(),        // Empty list is false
            Value::Map(m) => !m.is_empty(),         // Empty map is false
        }
    }
}
```

### Built-in Function Implementation

Functions are matched by name in the evaluation phase:

```rust
match func.as_str() {
    "echo" => {
        // Print formatted values
        if let Some(arg) = evaluated_args.first() {
            println!("{}", self.format_value(arg));
        }
        Ok(Value::Nil)
    }
    "add" => {
        // Perform arithmetic addition
        let result = self.arithmetic_op(&evaluated_args, |a, b| a + b)?;
        Ok(Value::Number(result))
    }
    // ... other functions
}
```

### Error Handling

The language uses string-based error messages:

- **Parse Errors**: Invalid syntax, unexpected tokens
- **Runtime Errors**: Undefined variables, wrong argument types
- **Type Errors**: Using non-numbers in arithmetic operations

### Performance Optimizations

1. **hashbrown**: High-performance HashMap implementation
2. **Zero-cost abstractions**: No runtime overhead for language features
3. **Efficient tokenization**: Single-pass character processing
4. **Minimal allocations**: Reuse memory where possible

---

## File Extensions

### Supported Extensions

- **`.i`** - Primary source file extension
- **`.rag`** - Alternative extension for compatibility

### File Validation

The interpreter validates file extensions before execution:

```rust
if !matches!(file_extension, "i" | "rag") {
    eprintln!("Error: i language only supports .i and .rag file extensions");
    process::exit(1);
}
```

---

## Advanced Usage

### Nested Expressions
```lisp
;; Complex nested calculation
(echo (mul (add 1 2) (sub 10 5)))  ;; (1+2) * (10-5) = 3 * 5 = 15
```

### Truthiness in Conditions
```lisp
;; Various truthiness examples
(echo (if "hello" "non-empty" "empty"))     ;; "non-empty"
(echo (if "" "non-empty" "empty"))          ;; "empty"
(echo (if [1] "has-items" "no-items"))      ;; "has-items"
(echo (if [] "has-items" "no-items"))       ;; "no-items"
```

### Variable-like Usage
```lisp
;; Using identifiers (will error unless defined)
(echo undefined_var)  ;; Error: Undefined variable
```

---

## Best Practices

1. **Use meaningful names** for functions and variables
2. **Leverage Zen constants** (`zen`, `flow`, `void`) for clarity
3. **Chain operations** with nested function calls
4. **Use `echo` liberally** for debugging and output
5. **Keep expressions simple** and readable
6. **Test truthiness** with different data types

---

## Language Limitations

1. **No user-defined functions** (only built-ins)
2. **No variables** (only constants and literals)
3. **No loops** (use recursion when available)
4. **No file I/O** (except reading source files)
5. **No error handling** (program exits on error)
6. **No modules** (single file execution)

These limitations are intentional, maintaining the language's minimalist philosophy while providing essential functionality.

---

*This guide covers every aspect of the i programming language. For specific implementation details, refer to the source code in `src/main.rs`.*
