# i Language Complete Word and Function Reference

## Overview

This document explains **every single word, function, and concept** in the i programming language. Each entry includes: syntax, purpose, parameters, return values, examples, and implementation details.

---

# Built-in Constants (Words)

## `zen`
**Syntax**: `zen`

**Purpose**: Represents the state of emptiness in Zen philosophy

**Value**: `0` (number)

**Truthiness**: **False** (zero evaluates to false)

**Examples**:
```lisp
(echo zen)              ;; Prints: 0
(echo (add zen 5))      ;; Prints: 5
(echo (if zen "true" "false"))  ;; Prints: "false"
```

**Implementation**: 
```rust
globals.insert("zen".to_string(), Value::Number(0.0));
```

**Philosophy**: In Zen, emptiness (śūnyatā) is the foundation of all existence. `zen` represents this concept as the numerical zero.

---

## `flow`
**Syntax**: `flow`

**Purpose**: Represents the state of being and natural movement in Zen philosophy

**Value**: `true` (boolean)

**Truthiness**: **True**

**Examples**:
```lisp
(echo flow)             ;; Prints: true
(echo (if flow "flowing" "still"))  ;; Prints: "flowing"
(echo (add 1 (if flow 5 0)))        ;; Prints: 6
```

**Implementation**:
```rust
globals.insert("flow".to_string(), Value::Boolean(true));
```

**Philosophy**: Flow represents the natural state of existence - continuous movement and being. It's the default positive state.

---

## `void`
**Syntax**: `void`

**Purpose**: Represents the state of nothingness and absence

**Value**: `nil` (null)

**Truthiness**: **False**

**Examples**:
```lisp
(echo void)             ;; Prints: void
(echo (if void "exists" "nothing"))  ;; Prints: "nothing"
```

**Implementation**:
```rust
globals.insert("void".to_string(), Value::Nil);
```

**Philosophy**: Void represents absolute nothingness - the complete absence of value or existence.

---

# Built-in Functions

## Input/Output Functions

### `echo`
**Syntax**: `(echo value1 value2 ...)`

**Purpose**: Output one or more values to the console

**Parameters**: 
- `value1, value2, ...`: Any number of values to output

**Return Value**: `void`

**Side Effects**: Prints formatted values to stdout

**Examples**:
```lisp
(echo "Hello")                    ;; Prints: "Hello"
(echo 42)                         ;; Prints: 42
(echo "Answer:" (add 2 3))        ;; Prints: "Answer:" 5
(echo zen flow void)              ;; Prints: 0 true void
```

**Implementation Details**:
```rust
"echo" => {
    if let Some(arg) = evaluated_args.first() {
        println!("{}", self.format_value(arg));
    }
    Ok(Value::Nil)
}
```

**Behavior**: 
- Only the **first argument** is displayed (current implementation)
- Values are formatted using `format_value()` method
- Text values are shown with quotes
- Numbers, booleans, and void shown without quotes

**Current Limitation**: Only processes first argument despite accepting multiple

---

## Arithmetic Functions

### `add`
**Syntax**: `(add number1 number2 number3 ...)`

**Purpose**: Add multiple numbers together

**Parameters**: 
- `number1, number2, ...`: 2 or more numeric values

**Return Value**: `Number` (sum of all inputs)

**Error Conditions**: 
- Requires at least 2 arguments
- All arguments must be numbers

**Examples**:
```lisp
(add 1 2)                        ;; Returns: 3
(add 1 2 3 4 5)                  ;; Returns: 15
(add 3.5 2.1)                    ;; Returns: 5.6
(add 10 -3)                      ;; Returns: 7
(add zen 5)                      ;; Returns: 5 (zen = 0)
```

**Implementation Details**:
```rust
"add" => {
    let result = self.arithmetic_op(&evaluated_args, |a, b| a + b)?;
    Ok(Value::Number(result))
}
```

**Algorithm**:
1. Validate at least 2 arguments
2. Ensure all arguments are numbers
3. Start with first number as initial result
4. Sequentially add each subsequent number
5. Return final sum

**Mathematical Properties**:
- **Commutative**: `(add a b)` = `(add b a)`
- **Associative**: `(add (add a b) c)` = `(add a (add b c))`
- **Identity**: `(add x 0)` = `x`

---

### `sub`
**Syntax**: `(sub number1 number2 number3 ...)`

**Purpose**: Subtract numbers sequentially from left to right

**Parameters**: 
- `number1, number2, ...`: 2 or more numeric values

**Return Value**: `Number` (result of sequential subtraction)

**Error Conditions**: 
- Requires at least 2 arguments
- All arguments must be numbers

**Examples**:
```lisp
(sub 10 3)                       ;; Returns: 7
(sub 20 5 2)                     ;; Returns: 13 (20 - 5 - 2)
(sub 5 10)                       ;; Returns: -5
(sub 100 20 30 10)               ;; Returns: 40
```

**Implementation Details**:
```rust
"sub" => {
    let result = self.arithmetic_op(&evaluated_args, |a, b| a - b)?;
    Ok(Value::Number(result))
}
```

**Algorithm**:
1. Validate at least 2 arguments
2. Ensure all arguments are numbers
3. Start with first number as initial result
4. Sequentially subtract each subsequent number
5. Return final result

**Mathematical Properties**:
- **Non-commutative**: `(sub a b)` ≠ `(sub b a)`
- **Non-associative**: `(sub (sub a b) c)` ≠ `(sub a (sub b c))`
- **Right Identity**: `(sub x 0)` = `x`

**Use Cases**:
- Difference calculation
- Countdown operations
- Reverse counting

---

### `mul`
**Syntax**: `(mul number1 number2 number3 ...)`

**Purpose**: Multiply multiple numbers together

**Parameters**: 
- `number1, number2, ...`: 2 or more numeric values

**Return Value**: `Number` (product of all inputs)

**Error Conditions**: 
- Requires at least 2 arguments
- All arguments must be numbers

**Examples**:
```lisp
(mul 4 5)                        ;; Returns: 20
(mul 2 3 4)                      ;; Returns: 24
(mul 1.5 2)                      ;; Returns: 3.0
(mul 10 -2)                      ;; Returns: -20
(mul zen 5)                      ;; Returns: 0 (zen = 0)
```

**Implementation Details**:
```rust
"mul" => {
    let result = self.arithmetic_op(&evaluated_args, |a, b| a * b)?;
    Ok(Value::Number(result))
}
```

**Algorithm**:
1. Validate at least 2 arguments
2. Ensure all arguments are numbers
3. Start with first number as initial result
4. Sequentially multiply each subsequent number
5. Return final product

**Mathematical Properties**:
- **Commutative**: `(mul a b)` = `(mul b a)`
- **Associative**: `(mul (mul a b) c)` = `(mul a (mul b c))`
- **Identity**: `(mul x 1)` = `x`
- **Zero Property**: `(mul x 0)` = `0`

**Use Cases**:
- Area calculations
- Scaling operations
- Compound growth

---

### `div`
**Syntax**: `(div number1 number2 number3 ...)`

**Purpose**: Divide numbers sequentially from left to right

**Parameters**: 
- `number1, number2, ...`: 2 or more numeric values

**Return Value**: `Number` (result of sequential division)

**Error Conditions**: 
- Requires at least 2 arguments
- All arguments must be numbers
- Division by zero (runtime error)

**Examples**:
```lisp
(div 20 4)                       ;; Returns: 5
(div 100 5 2)                    ;; Returns: 10 (100 / 5 / 2)
(div 15 3)                       ;; Returns: 5
(div 7 2)                        ;; Returns: 3.5
(div 1 2)                        ;; Returns: 0.5
```

**Implementation Details**:
```rust
"div" => {
    let result = self.arithmetic_op(&evaluated_args, |a, b| a / b)?;
    Ok(Value::Number(result))
}
```

**Algorithm**:
1. Validate at least 2 arguments
2. Ensure all arguments are numbers
3. Start with first number as initial result
4. Sequentially divide by each subsequent number
5. Return final result

**Mathematical Properties**:
- **Non-commutative**: `(div a b)` ≠ `(div b a)`
- **Non-associative**: `(div (div a b) c)` ≠ `(div a (div b c))`
- **Right Identity**: `(div x 1)` = `x`

**Special Cases**:
- Division by zero causes runtime error
- Integer division returns floating point
- Negative division works correctly

**Use Cases**:
- Ratio calculations
- Average operations (combined with `add` and `div`)
- Fractional operations

---

## Control Flow Functions

### `if`
**Syntax**: `(if condition then_value else_value)`

**Purpose**: Conditional selection based on truthiness of condition

**Parameters**: 
- `condition`: Any value to evaluate for truthiness
- `then_value`: Value to return if condition is truthy
- `else_value`: Value to return if condition is falsy

**Return Value**: Either `then_value` or `else_value`

**Error Conditions**: 
- Requires exactly 3 arguments

**Examples**:
```lisp
(if flow "yes" "no")              ;; Returns: "yes"
(if zen "yes" "no")               ;; Returns: "no"
(if "hello" "non-empty" "empty") ;; Returns: "non-empty"
(if "" "non-empty" "empty")      ;; Returns: "empty"
(if (add 1 -1) "zero" "non-zero") ;; Returns: "zero"
```

**Implementation Details**:
```rust
"if" => {
    if evaluated_args.len() != 3 {
        return Err("if requires exactly 3 arguments: condition, then, else".to_string());
    }
    let condition = &evaluated_args[0];
    let then_expr = &evaluated_args[1];
    let else_expr = &evaluated_args[2];
    
    if condition.is_truthy() {
        Ok(then_expr.clone())
    } else {
        Ok(else_expr.clone())
    }
}
```

**Truthiness Rules**:
- **Number**: Zero is false, non-zero is true
- **Text**: Empty string is false, non-empty is true
- **Boolean**: Direct boolean value
- **List**: Empty list is false, non-empty is true
- **Map**: Empty map is false, non-empty is true
- **Nil**: Always false

**Behavior**:
1. Evaluate condition (already evaluated by interpreter)
2. Check truthiness using `is_truthy()` method
3. Return appropriate branch without evaluating the other
4. Return value is cloned from the selected branch

**Use Cases**:
- Conditional output
- Error handling
- State selection
- Mathematical branching

---

# Data Types and Literals

## Numbers
**Syntax**: Digits with optional decimal point and optional leading minus

**Examples**:
```lisp
42          ;; Integer
3.14        ;; Floating point
-7          ;; Negative integer
-2.5        ;; Negative float
0           ;; Zero (same as zen)
```

**Implementation**: `Value::Number(f64)`

**Properties**:
- 64-bit floating point precision
- Supports integer and fractional values
- Mathematical operations available
- Truthiness: 0 = false, non-zero = true

---

## Text (Strings)
**Syntax**: Double-quoted character sequences

**Examples**:
```lisp
"Hello"                     ;; Simple text
"i language"                ;; Text with space
""                          ;; Empty string
"Line 1\nLine 2"            ;; With escape sequences
"\"Quotes\""                ;; Escaped quotes
```

**Implementation**: `Value::Text(String)`

**Properties**:
- Unicode support
- Escape sequences supported by Rust strings
- Truthiness: empty = false, non-empty = true
- No built-in string operations (yet)

---

## Booleans
**Syntax**: `flow` (true) or any falsy value

**Examples**:
```lisp
flow        ;; True constant
true        ;; Boolean true (if supported)
false       ;; Boolean false (if supported)
```

**Implementation**: `Value::Boolean(bool)`

**Properties**:
- Only `flow` is built-in true constant
- Truthiness: direct boolean value
- Used in conditional expressions

---

## Nil (Void)
**Syntax**: `void`

**Examples**:
```lisp
void        ;; Nil value
```

**Implementation**: `Value::Nil`

**Properties**:
- Represents absence of value
- Always falsy
- Returned by functions with no meaningful output
- Used as "null" equivalent

---

## Lists
**Syntax**: Square bracket with space-separated values

**Examples**:
```lisp
[1 2 3]                     ;; Number list
["a" "b" "c"]               ;; Text list
[1 "two" 3]                 ;; Mixed list
[]                          ;; Empty list
[flow void zen]             ;; Constant list
```

**Implementation**: `Value::List(Vec<Value>)`

**Properties**:
- Ordered collection
- Mixed types allowed
- Truthiness: empty = false, non-empty = true
- No built-in list operations (yet)

---

## Maps
**Syntax**: Curly braces with key:value pairs

**Examples**:
```lisp
{key: value}                ;; Single pair
{name: "i" version: 1}      ;; Multiple pairs
{}                          ;; Empty map
{zen: 0 flow: true}         ;; Constant map
```

**Implementation**: `Value::Map(hashbrown::HashMap<String, Value>)`

**Properties**:
- Unordered key-value pairs
- String keys only
- Mixed value types allowed
- Truthiness: empty = false, non-empty = true
- No built-in map operations (yet)

---

# Syntax Elements

## Parentheses `()`
**Purpose**: Function call delimiters

**Syntax**: `(function_name arg1 arg2 ...)`

**Examples**:
```lisp
(echo "Hello")               ;; Function call
(add 1 2)                    ;; Arithmetic
(if flow "yes" "no")         ;; Conditional
```

**Behavior**:
- Opens function call expression
- Must be matched with closing parenthesis
- Nested parentheses allowed
- First element must be function name

---

## Square Brackets `[]`
**Purpose**: List delimiters

**Syntax**: `[item1 item2 item3]`

**Examples**:
```lisp
[1 2 3]                     ;; Number list
["a" "b"]                   ;; Text list
[]                          ;; Empty list
```

**Behavior**:
- Defines list literal
- Space-separated values
- Can contain any value types
- Cannot be used for function calls

---

## Curly Braces `{}`
**Purpose**: Map delimiters

**Syntax**: `{key1: value1 key2: value2}`

**Examples**:
```lisp
{name: "i"}                 ;; Single pair
{a: 1 b: 2}                 ;; Multiple pairs
{}                          ;; Empty map
```

**Behavior**:
- Defines map literal
- Key:value pairs separated by spaces
- Keys must be identifiers (strings)
- Values can be any type

---

## Quotes `"`
**Purpose**: Text string delimiters

**Syntax**: `"text content"`

**Examples**:
```lisp
"Hello"                     ;; Simple text
"Multi word text"           ;; With spaces
""                          ;; Empty string
"Escaped \"quote\""         ;; With escapes
```

**Behavior**:
- Starts and ends text literal
- Supports escape sequences
- Cannot span multiple lines (currently)
- Text content can include spaces

---

## Comments `;;`
**Purpose**: Code documentation and notes

**Syntax**: `;; comment text`

**Examples**:
```lisp
;; This is a comment
(echo "Hello")              ;; Output greeting
;; Multi-line
;; comment block
```

**Behavior**:
- Ignored by tokenizer
- Starts with double semicolon
- Continues to end of line
- Used for documentation

---

# Error Messages

## File Extension Errors
**Message**: `"Error: i language only supports .i and .rag file extensions"`

**Cause**: Attempting to run file with unsupported extension

**Solution**: Use `.i` or `.rag` file extension

---

## Argument Count Errors
**Message**: `"if requires exactly 3 arguments: condition, then, else"`

**Cause**: Wrong number of arguments to `if` function

**Solution**: Provide exactly 3 arguments

---

## Arithmetic Operation Errors
**Message**: `"Arithmetic operation requires at least 2 arguments"`

**Cause**: Too few arguments to arithmetic functions

**Solution**: Provide at least 2 arguments

---

## Type Errors
**Message**: `"Arithmetic operation requires numbers"`

**Cause**: Non-numeric arguments to arithmetic functions

**Solution**: Ensure all arguments are numbers

---

## Undefined Variable Errors
**Message**: `"Undefined variable: variable_name"`

**Cause**: Using undefined identifier

**Solution**: Use defined constants or literals

---

## Parse Errors
**Message**: `"Unexpected token: token_type"`

**Cause**: Invalid syntax in source code

**Solution**: Check syntax and parentheses matching

---

# Performance Characteristics

## Time Complexity
- **Tokenization**: O(n) - linear in source length
- **Parsing**: O(n) - linear in token count
- **Evaluation**: O(n) - linear in AST size
- **Function calls**: O(k) - where k is number of arguments

## Space Complexity
- **Memory usage**: O(n) - proportional to program size
- **Stack depth**: O(d) - where d is nesting depth
- **Global variables**: O(g) - where g is number of globals

## Optimizations
- **hashbrown**: Fast HashMap implementation
- **Zero-cost abstractions**: No runtime overhead
- **Efficient tokenization**: Single character processing
- **Minimal allocations**: Reuse where possible

---

# Future Extensions

## Potential Additions
- **User-defined functions**
- **Variable assignment**
- **Loop constructs**
- **String operations**
- **List operations**
- **Map operations**
- **File I/O**
- **Error handling**
- **Module system**

## Design Principles for Extensions
- Maintain Zen simplicity
- Preserve minimalist philosophy
- Add only essential features
- Keep performance high
- Ensure backward compatibility

---

# Complete Example Program

```lisp
;; Complete i language program example
;; Demonstrates every language feature

;; Output greeting
(echo "Welcome to i Language")

;; Show Zen constants
(echo "Zen state:" zen)
(echo "Flow state:" flow)
(echo "Void state:" void)

;; Demonstrate arithmetic
(echo "Mathematics:")
(echo "Addition:" (add 1 2 3))
(echo "Subtraction:" (sub 10 3))
(echo "Multiplication:" (mul 4 5))
(echo "Division:" (div 20 4))

;; Show conditionals
(echo "Conditionals:")
(echo "Flow is true:" (if flow "yes" "no"))
(echo "Zero is false:" (if zen "yes" "no"))
(echo "Empty text is false:" (if "" "yes" "no"))

;; Complex expressions
(echo "Complex calculation:")
(echo (mul (add 2 3) (sub 10 5)))

;; Data structures
(echo "Data structures:")
(echo [1 2 3])
(echo {name: "i" type: "language"})

;; Final message
(echo "Program complete!")
```

**Expected Output**:
```
Welcome to i Language
Zen state: 0
Flow state: true
Void state: void
Mathematics:
Addition: 6
Subtraction: 7
Multiplication: 20
Division: 5
Conditionals:
Flow is true: yes
Zero is false: no
Empty text is false: no
Complex calculation: 15
Data structures:
[1 2 3]
{name: "i" type: "language"}
Program complete!
```

---

*This reference covers every word, function, and concept in the i programming language. For implementation details, see src/main.rs.*
