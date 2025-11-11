# Rust Structs - Learning Notes

## Basic Struct Definition and Usage

```rust
fn main() {
    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        email: String,
    }
    
    let user = User {
        id: 1,
        name: String::from("Mohit"),
        email: String::from("john.doe@example.com"),
    };
    
    println!("User ID: {}", user.id);
    println!("{:?}", user);
    
    // Note: {:?} is not the correct way to print the user struct
    // To print the user struct properly, use {:#?} instead of {:?} format specifier
}
```

**Key Points:**
- `struct User` is a **struct declaration** (defines the structure)
- `user` is an **instance** of the User struct (actual data)
- `#[derive(Debug)]` enables debug printing with `{:?}` or `{:#?}`
- `{:#?}` provides pretty-printed output (recommended)

---

## Program Using Structs - Rectangle Area Example

### Version 1: Without Structs

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

### Version 2: With Structs (Better Approach)

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("{}", area(rect1));
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

**Benefits of using structs:**
- Groups related data together
- Makes code more readable and maintainable
- Type safety - width and height are always together

---

## The `dbg!` Macro - Debugging Helper

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}
```

### What `dbg!` Does:

**Line 1:** `dbg!(30 * scale)` prints:
```
[src/main.rs:9] 30 * scale = 60
```
- Returns `60`, which becomes the value of `width`

**Line 2:** `dbg!(&rect1)` prints:
```
[src/main.rs:13] &rect1 = Rectangle { width: 60, height: 50 }
```

### Key Features of `dbg!`:
- Shows **file name** and **line number** automatically
- Prints the **expression** and its **value**
- **Returns the value**, so it can be used inline
- Acts like a built-in debugger print statement
- Use `&` when you don't want to take ownership (like `&rect1`)

**Perfect for quick debugging without disrupting your code flow!**