# Rust Enums - Complete Guide

## Table of Contents
1. [Structs vs Enums](#structs-vs-enums)
2. [Real-World Example: IP Addresses](#real-world-example-ip-addresses)
3. [Basic Enum Definition](#basic-enum-definition)
4. [Using Enums](#using-enums)
5. [Enums with Data](#enums-with-data)
6. [Functions with Enums](#functions-with-enums)
7. [Evolution of IP Address Implementation](#evolution-of-ip-address-implementation)

---

## Structs vs Enums

### Structs
- Group **multiple fields** together
- All fields exist simultaneously
- Example: A `Rectangle` with `width` AND `height`

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

### Enums
- Define a type that can be **one of several possible values**
- Only **one variant** exists at a time
- Example: An IP address is EITHER IPv4 OR IPv6 (never both)

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

---

## Real-World Example: IP Addresses

### The Problem
There are only **two valid kinds** of IP addresses:
- **IPv4** (version 4)
- **IPv6** (version 6)

A single IP address **can't be both** at the same time.

### Wrong Approach ❌
Using two separate structs:

```rust
struct Ipv4Addr { /* fields */ }
struct Ipv6Addr { /* fields */ }
```

This doesn't enforce that an address is one or the other.

### Right Approach ✅
Use one enum that represents either variant:

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

---

## Basic Enum Definition

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

**Key Points:**
- `IpAddrKind` is a **custom data type**
- It can only be **one of two variants**: `V4` or `V6`
- This is called an **enumeration** (enum for short)

---

## Using Enums

### Creating Values

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

**Important:**
- Both `four` and `six` are of type `IpAddrKind`
- They're just **different variants** of the same type
- Any function accepting `IpAddrKind` can handle **either** IPv4 or IPv6 uniformly

---

## Enums with Data

### Tuple Style (Simpler)

Each variant can **hold multiple values**:

```rust
enum IpAddr {
    V4(String, u8, u8, u8),  // address + 3 numbers
    V6(String, String),
}
```

**Example Usage:**

```rust
let ipv4 = IpAddr::V4(String::from("192.168.1.1"), 255, 255, 0);
let ipv6 = IpAddr::V6(String::from("::1"), String::from("localhost"));
```

---

## Functions with Enums

### Basic Function

```rust
fn route(ip_kind: IpAddrKind) {}
```

This function accepts **either** of the two enum variants:
- `IpAddrKind::V4`
- `IpAddrKind::V6`

**Benefit:** Enums let you pass a type with **defined possible values** instead of arbitrary strings or numbers.

---

## Evolution of IP Address Implementation

### Version 1: Struct + Enum (Verbose)

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
```

**How it works:**
- The **enum** tells which type of address it is (IPv4 or IPv6)
- The **struct** stores the actual address string

**Usage:**

```rust
let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};
```

**Drawback:** Slightly verbose — you always need both `kind` and `address`.

---

### Version 2: Simplified with Only an Enum ⭐

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

**Improvements:**
- `V4` and `V6` **directly store the data**
- Removes the need for a separate struct
- Much cleaner and more concise!

**Usage:**

```rust
let home = IpAddr::V4(String::from("127.0.0.1"));
```

**Magic:** Rust automatically treats each variant as a **constructor function**:
- `IpAddr::V4()` is a function that returns an `IpAddr` value

**Important Clarification:**
- An enum is **not a function** — it's a **data type** (like a `struct` or `int`)
- But it has **multiple possible forms** (variants)
- However, each variant of an enum can **act like a constructor function** because it can create a value of that type

---

### Version 3: Different Data Types Per Variant 🚀

Unlike structs (which have a fixed layout), **enum variants can each store different kinds of data**:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

**Example:**

```rust
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

**Key Points:**
- `V4` stores **four numbers** (u8, u8, u8, u8)
- `V6` stores **one string**
- This **flexibility** is why enums are more powerful than structs in such cases

---

### Version 4: Standard Library Implementation 📚

Rust's standard library defines it like this:

```rust
struct Ipv4Addr { /* fields */ }
struct Ipv6Addr { /* fields */ }

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

**How it works:**
- Each variant holds a **different struct**
- Gives **fine-grained control** over both types of addresses
- This is the real implementation behind `std::net::IpAddr`

**Benefits:**
- Maximum flexibility
- Type safety
- Each IP version can have its own specific methods and fields

---

## Summary

| Feature | Structs | Enums |
|---------|---------|-------|
| **Purpose** | Group related data together | Represent one of several variants |
| **Fields** | All fields exist at once | Only one variant exists at a time |
| **Data Storage** | Fixed structure | Each variant can store different types |
| **Use Case** | When you need ALL the data | When you need ONE of several options |

**When to use Enums:**
✅ When a value can only be one of a fixed set of options
✅ When different cases need different data
✅ When you want type-safe alternatives to magic numbers/strings

**Key Takeaway:** Enums are Rust's way of saying "this value is one of these specific things" — making your code safer and more expressive! 🦀