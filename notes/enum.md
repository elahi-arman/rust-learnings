# Enumerations
We express enums in the following way. Each of the different values it can take are called variants.

``` rust
enum IpAddrKind {
  V4,
  V6
}

let four = IpAddrKind::V4;

rn route(ip_type: IpAddrKind){}
```

We can put data directly inside an enum variant. This would take the place of a struct with a variant defined + the data that wouold be wanted.

```Rust

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

You can put in any data that you want into the enum. Even complex struct types. Which allows us to group structs into an enum:

``` rust
enum Message {
    Quit,                       // unit struct
    Move { x: i32, y: i32 },    // anonymous struct
    Write(String),              // struct tuple      
    ChangeColor(i32, i32, i32), // struct tuple
}
```

We can even define methods on enums. So in general, enums are much more powerful than in any other language.

# Option Enum

The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing. Expressing this concept in terms of the type system means the compiler can check that you’ve handled all the cases you should be handling, which can prevent bugs that are extremely common in other programming languages.

The problem with null values is that if you try to actually use a value that’s null as if it is a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

<T> is a generic type parameter -- so it can hold one piece of any typed data. the compiler won’t let us use an Option<T> value as if it was definitely a valid value. , you have to convert an Option<T> to a T before you can perform T operations with it.

``` rust
enum Option<T> {
    Some(T),
    None,
}
```

In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.

The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

# Match

Match expressions can return any type, as opposed to if conditions which would return a boolean. The patterns are executed in order, so it's like a switch case with a break after every pattern.

We can also match partial patterns and extract useful information

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

Useful pattern: match against an enum, bind a variable to the data inside, and then execute code based on it. Especially since this is exhaustive, we can be sure that the compiler will force us to catch all variants. To match anything note listed before, we use the _ variable. And we can return (), a unit value.

# If Let

if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values. We can then use an else block to catch the _ case.
