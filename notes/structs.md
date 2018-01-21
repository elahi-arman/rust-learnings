# Introduction
structs -- custom data type that lets us name and package related values. This is like the an object's data attributes. We create instances of structs by specifying concrete values for each field.

The entire instances of a struct must be mutable. There's a structu field init shorthand, like in JS

```Rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

Building structs from old values of structs is pretty easy too.

```Rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

We can also name tuples  without specifying names for the fields. Each struct defines its own type even if the field types are the same

```rust
struct Color (i32, i32, i32);
```

Instances of a struct own all their data. So we should use Strings as opposed to &str. Structs can store references to data owned by something else, but that requires the use of a lifetime

Printing Structures
---

To print out structure, we have to use the {:?} or {#?} (pretty print) syntax in our println! macro. But we need to opt in to this functionality for a struct by using an annotation

``` Rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}
```

Methods
---
Methods are just functions defined in the context of a structs, enums, or traits. THeir first parameter is always self, a reference to the instance of the struct.

Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and we want to prevent the caller from using the original instance after the transformation.

``` Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation block for a struct
impl Rectangle {
    // methods can take ownership of self or borrow (im)mutably
    // since we don't want to modify, we just borrow it immutably
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Rust has auto referencing and auto dereferncing. So don't need to worry about that on methods.

We can also create associated functions, like static functions. They're often used as constructors and use the `::` with the struct name.

There can be multiple impl blocks per struct, so we can extend functionality among different files 
