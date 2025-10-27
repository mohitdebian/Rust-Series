# Day 9 of My Rust Journey - Structs and Methods

## Introduction

Structs in Rust are custom data types that let you package related data together. Combined with `impl` blocks, they allow you to define both data and behavior, similar to classes in other languages but with Rust's unique approach.

**Topics Covered:**
- What structs are
- Defining struct fields
- Understanding `impl` blocks
- Creating methods with `&self`
- Instantiating and using structs

---

## What is a Struct?

A struct in Rust is a **data container**. It only defines what data something has — **no behavior**.

### Basic Syntax

```rust
struct User {
    name: String,
    age: u8,
}
```

### Explanation

- `struct` keyword defines a new custom type
- `User` is the name of our struct (convention: PascalCase)
- Inside curly braces, we define **fields**:
  - `name` is a `String` (heap-allocated)
  - `age` is a `u8` (unsigned 8-bit integer, 0-255)
- This defines the **structure** but doesn't create any instances yet

**Key Point:** At this stage, `User` is just a blueprint. You can't make it "do" anything yet.

---

## What is an `impl` Block?

An `impl` block adds **behavior** (functions) to a struct. That's where you write methods that work on the struct's data.

### Syntax

```rust
impl User {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}
```

### Explanation

- `impl User` means "implementation for User struct"
- Inside, we define **methods** that belong to `User`
- `fn greet(&self)` defines a method named `greet`
- `&self` is a reference to the current instance (like `this` in other languages)
- `self.name` accesses the `name` field of the current `User` instance

**Key Point:** Now every `User` can call the `greet()` method!

---

## Understanding `&self`

The `&self` parameter is special:

- **`&self`** = immutable reference to the instance
- **`&mut self`** = mutable reference (can modify fields)
- **`self`** = takes ownership (consumes the instance)

In our example, `&self` means the method can **read** the struct's data but not modify it.

---

## Putting It All Together

### Complete Example

```rust
struct User {
    name: String,
    age: u8,
}

impl User {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let user = User {
        name: String::from("Mohit"),
        age: 19,
    };

    user.greet(); // Calls the method defined in impl
}
```

### Output

```
Hello, Mohit!
```

### Step-by-Step Breakdown

1. **Define the struct** - Create the `User` blueprint with `name` and `age` fields

2. **Add behavior** - Use `impl User` to add the `greet()` method

3. **Create an instance** - In `main()`, instantiate a `User` with specific values:
   - `name: String::from("Mohit")` creates a heap-allocated String
   - `age: 19` sets the age value

4. **Call the method** - `user.greet()` invokes the method, which accesses `self.name`

---

## Key Differences from Other Languages

### Rust vs Classes in OOP Languages

| Feature | Rust | Java/C++ |
|---------|------|----------|
| Data definition | `struct` | `class` |
| Methods | `impl` block | Inside class |
| Self reference | Explicit `&self` | Implicit `this` |
| Mutability | Explicit `&mut self` | Always mutable |

---

## Creating Struct Instances

### Syntax Pattern

```rust
let instance_name = StructName {
    field1: value1,
    field2: value2,
};
```

### Rules

✅ All fields must be initialized

✅ Field order doesn't matter

✅ Use same names as defined in struct

✅ Values must match field types

---

## Field Init Shorthand

When creating a struct, if your variable names exactly match the struct's field names, you can skip the repetition.

### Without Shorthand

```rust
User {
    username: username,  // redundant!
    email: email,        // redundant!
}
```

### With Shorthand

```rust
User {
    username,  // Rust knows you mean username: username
    email,     // Rust knows you mean email: email
}
```

### Explanation

- This is purely **syntactic sugar** — it works exactly the same way, just cleaner to read
- Rust automatically assigns the variable value to the field with the same name
- Makes code more concise when variable and field names match

---

## Struct Update Syntax (the `..` operator)

This lets you create a new struct by copying most fields from an existing instance and only specifying the fields you want to change.

### Example

```rust
let user2 = User {
    email: String::from("another@example.com"),  // New value
    ..user1  // Copy everything else from user1
};
```

### What It Expands To

This is equivalent to manually writing out all the other fields:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    active: user1.active,
    username: user1.username,
    sign_in_count: user1.sign_in_count,
};
```

### The Ownership Catch ⚠️

The `..` syntax follows Rust's ownership rules.

**What gets moved vs copied:**

- Types that implement `Copy` (like `bool`, `i32`, `f64`) are **copied** → `user1` still owns them
- Types that don't implement `Copy` (like `String`, `Vec`) are **moved** → `user1` loses ownership

### Complete Example

```rust
let user1 = User {
    active: true,           // bool - implements Copy
    username: String::from("user1"),  // String - does NOT implement Copy
    email: String::from("user1@example.com"),
    sign_in_count: 1,       // i32 - implements Copy
};

let user2 = User {
    email: String::from("user2@example.com"),
    ..user1
};

// After this:
// ✅ user1.active is still usable (was copied)
// ✅ user1.sign_in_count is still usable (was copied)
// ❌ user1.username is NOT usable (was moved to user2)
// ✅ user1.email is still usable (we didn't use it in the update)
```

### Key Points

✅ **`..user1` must come last** in the struct initialization

✅ **Only unspecified fields** are taken from the source struct

✅ **Ownership rules apply** - non-Copy types are moved

✅ **Partial usability** - you can still use Copy fields from the original struct after the update

---

## Best Practices

- **Struct names**: Use PascalCase (e.g., `User`, `BankAccount`)
- **Field names**: Use snake_case (e.g., `first_name`, `account_balance`)
- **Method names**: Use snake_case (e.g., `greet`, `calculate_age`)
- **Separate concerns**: Data in struct, behavior in `impl`
- **Use `&self`**: Default to immutable references unless you need to modify

---

## Common Patterns

### Multiple Methods

```rust
impl User {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
    
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    fn have_birthday(&mut self) {
        self.age += 1;
    }
}
```

### Associated Functions (No `self`)

```rust
impl User {
    fn new(name: String, age: u8) -> User {
        User { name, age }
    }
}

// Usage
let user = User::new(String::from("Mohit"), 19);
```

---

## Key Insights

✅ **Structs = Data**: Define what something has

✅ **impl = Behavior**: Define what something can do

✅ **`&self` = Access**: Methods can read the instance's data

✅ **Separation**: Clear distinction between data structure and functionality

✅ **Type Safety**: Compiler ensures all fields are initialized correctly

---

## Next Steps

Topics to explore:
- Tuple structs and unit-like structs
- Struct update syntax
- Lifetime annotations in structs
- Deriving traits (`Debug`, `Clone`, etc.)
- Generic structs