# Day 6 - Mutable References and Borrowing Rules

## Introduction

Mutable references allow you to change the value of a variable through a reference while still enforcing Rust's ownership rules. Rust ensures **memory safety** by restricting simultaneous mutable access to data.

**Topics Covered:**
- Creating mutable references
- Scope and lifetime of references
- Rules for mutable and immutable references
- Reference validity and ownership

---

## Mutable Reference Scope

### Code Example

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

### Explanation

- `s` is a mutable `String`
- `r1` is a mutable reference to `s` inside the inner block
- Once the block ends, `r1` goes out of scope
- This allows a new mutable reference `r2` to be created safely
- Rust ensures no two mutable references exist at the same time to prevent data races and undefined behavior

---

## Invalid Example: Two Mutable References Simultaneously

### Code Example

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{r1}, {r2}");
```

### Explanation

- `s` is a mutable `String`
- `r1` is a mutable reference to `s`
- `r2` attempts to create another mutable reference to `s` at the same time
- **Rust forbids this** because having multiple mutable references simultaneously can cause memory safety issues
- The compiler will throw an error, stopping the code from compiling

**Key Point:** Only one mutable reference is allowed at a time.

---

## Reference Validity

| Status | Reference Type | Validity |
|--------|---------------|----------|
| ❌ | `&String` → reference to dropped data | **Invalid** |
| ✅ | `String` → ownership moved safely | **Valid** |

### Explanation

- Rust tracks lifetimes of variables and references
- References cannot outlive the data they point to
- This prevents dangling references, a common source of bugs in other languages

---

## The Rules of References

At any given time, you can have **either**:

1. **One mutable reference**, OR
2. **Any number of immutable references**

Additionally:

- References must always be valid

### Key Insights

- **Mutable reference** = exclusive access to the data
- **Immutable references** = shared read-only access
- Rust enforces these rules at compile time to guarantee memory safety

---

## Next Concept: Slices

**Slices** are a kind of reference that lets you access a portion of a collection without taking ownership.

**Use Cases:**
- Safely working with subsets of data
- Accessing parts of arrays or strings
- Avoiding unnecessary data copying
