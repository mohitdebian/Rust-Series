# Day 7 of My Rust Journey - Iterator `enumerate()` Method & String Slices

## Introduction

The `enumerate()` method is a powerful iterator adapter in Rust that wraps an iterator and provides both the index and value for each element. This eliminates the need for manual index tracking in loops.

**Topics Covered:**
- Understanding `enumerate()` method
- Working with tuples in iteration
- Practical use cases: Finding word boundaries
- String slices and borrowing rules

---

## What is `enumerate()`?

The `enumerate()` method:
- Wraps an existing iterator
- Returns a tuple `(index, value)` for each element
- Index starts from 0 by default
- Useful when you need both position and value

---

## Basic Example

### Code

```rust
let arr = [10, 20, 30];

for (i, &v) in arr.iter().enumerate() {
    println!("Index {} has value {}", i, v);
}
```

### Output

```
Index 0 has value 10
Index 1 has value 20
Index 2 has value 30
```

### Explanation

- `arr.iter()` creates an iterator over the array
- `.enumerate()` wraps it to provide `(index, value)` tuples
- `(i, &v)` destructures the tuple:
  - `i` is the index (usize)
  - `&v` borrows the value from the array
- Each iteration prints both index and corresponding value

---

## Practical Example: Finding First Space Index

### Code

```rust
fn main() {
    let mut s = String::from("Hello, World!"); // create a String on the heap with value "Hello, World!"
    let index = checking(&s);                   // call the function `checking` with a reference to `s` and store the returned value in `index`
    println!("The first space is at index {}", index); // print the returned index of the first space
}

fn checking(s: &String) -> usize {             // define a function that takes a reference to a String and returns a usize
    let bytes = s.as_bytes();                  // convert the String into a byte array (u8) to process each character
    for (i, &item) in bytes.iter().enumerate() { // iterate over bytes with index `i` and byte value `item`
        if item == b' ' {                      // check if the byte represents a space character (b' ' is a byte literal for space)
            return i;                          // if a space is found, immediately return its index
        }
    }
    s.len()                                     // if no space is found, return the length of the string (the whole string is one word)
}
```

### Key Points

✅ **`&s`** → passes a reference, so `checking` does not take ownership

✅ **`as_bytes()`** → allows iterating over the raw bytes instead of chars

✅ **`enumerate()`** → gives both index (`i`) and value (`item`) for each byte

✅ **`b' '`** → byte literal for space

✅ **`return i`** → sends the first space index back to the caller

✅ **`s.len()`** → fallback if no space is found

---

## Returning String Slices

### Code

```rust
fn first_word(s: &String) -> &str {             // function takes a reference to a String and returns a string slice (&str)
    let bytes = s.as_bytes();                  // convert the String into a byte array to examine each character
    for (i, &item) in bytes.iter().enumerate() { // iterate over bytes with index `i` and byte value `item`
        if item == b' ' {                      // check if the current byte is a space character (b' ' is a byte literal)
            return &s[0..i];                   // return a slice of the string from start (0) up to the space index `i`
        }
    }
    &s[..]                                      // if no space is found, return a slice of the entire string
}
```

### Key Points

✅ **`&String`** → pass by reference, function does not take ownership

✅ **`&str`** → the return type is a string slice, a view into the original string

✅ **`as_bytes()`** → lets us iterate over raw bytes, needed to compare with `b' '`

✅ **`enumerate()`** → gives both index and value during iteration

✅ **`&s[0..i]`** → creates a slice from start to first space, efficiently

✅ **`&s[..]`** → returns the whole string if there is no space

---

## Borrowing Rules with String Slices

### Code That Won't Compile

```rust
fn main() {
    let mut s = String::from("hello world"); // create a mutable String on the heap
    let word = first_word(&s);               // take an immutable reference to s and return a &str slice
                                             // word now **borrows s immutably**
    s.clear();                               // tries to borrow s **mutably** to empty it
                                             // ❌ ERROR: Rust forbids mutable borrow while immutable borrow exists
    println!("the first word is: {word}");  // word still uses the immutable borrow
}
```

### Why the Error Occurs

1. **`first_word(&s)`** creates an immutable reference (`&s`) that lives as long as `word` is used

2. **`s.clear()`** needs a mutable reference (`&mut s`) to modify the String

3. **Rust's borrowing rules:**
   - You cannot have a mutable reference while an immutable reference exists
   - Here, `word` (the immutable slice) is still active
   - The compiler detects this and stops compilation

### The Rule in Action

- **Immutable borrow** (`word`) is created first
- **Mutable borrow** (`s.clear()`) is attempted while immutable borrow is still in use
- **Result**: Compilation error ❌
- **Rust prevents**: Dangling references and use-after-free bugs

---

## Breaking Down the Syntax

### `arr.iter()`
Creates an iterator that borrows each element of the array.

### `.enumerate()`
Transforms the iterator to yield `(usize, &T)` tuples where:
- First element: index (0, 1, 2, ...)
- Second element: reference to the value

### `(i, &v)`
Pattern matching to destructure the tuple:
- `i` captures the index
- `&v` dereferences to get the actual value

---

## Key Insights

✅ **No Manual Indexing**: Eliminates error-prone manual counter management

✅ **Type Safety**: Index type is guaranteed to be `usize`

✅ **Iterator Composability**: Can be chained with other iterator methods

✅ **Zero-Cost Abstraction**: Compiles to efficient machine code

✅ **Memory Safety**: Borrowing rules prevent data races at compile time

---

## Best Practices

- Use `enumerate()` instead of manual index tracking
- Prefer pattern matching `(i, &v)` for clarity
- Combine with other iterator methods for powerful data processing
- Remember: indices are zero-based
- Be mindful of borrowing rules when working with slices

---

## Next Steps

Explore more iterator methods:
- `map()` - Transform each element
- `filter()` - Select elements based on conditions
- `zip()` - Combine two iterators
- `collect()` - Convert iterator back to collections