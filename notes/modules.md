# Modules

A namespace with functions and types that can be public or private. The main keywords:
  - mod => declares a new module, the code can be immediately following or in another file
  - pub => marks something public or else it's private.
  - use => brings modules or the definitions  into scope

To create a module, just run `cargo new <lib_name>`

The entry point to any module is the src/lib.rs. We can nest different modules inside of larger modules.

``` rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```


# File organization

- module named foo with no submodules, put declarations in foo.rs
- module named bar with submodule, put declarations in foo/mod.rs

To create more organized hierarchies, break shit into files. If we want to get this module structure.

communicator
 ├── client
 └── network
     └── client

We could have the following code

``` rust

// lib.rs
mod client; // replace block with semicolon means look in another location
mod network;

// network/mod.rs
fn connect() {
}
mod server;

// client.rs
fn connect() {
}

// network/client.rs
fn connect() {
}
```

In client.rs, we didn't need to have a mod declaration. If we put a mod client in there, then client would have a submodule called client.

For the network module, we would create src/network.rs. But we wouldn’t be able to extract the network::client module into a src/client.rs file because that already exists for the top-level client module! If we could put the code for both the client and network::client modules in the src/client.rs file, Rust wouldn’t have any way to know whether the code was for client or for network::client.

# Using libraries

You can have a binary crate live alongside a library crate by creating a src/main.rs file. Then to
call the library, use the ```extern crate <lib_name>``` in your main. even if we’re using an external crate within a submodule of our project, the extern crate should go in our root module (so in src/main.rs or src/lib.rs).

To make code public, you have to make the module and the inner function pub.

If an item is public, it can be accessed through any of its parent modules.
If an item is private, it can be accessed only by its immediate parent module and any of the parent’s child modules

# Referring ton Names in Diff Modules

The use keyword brings only what we’ve specified into scope: it does not bring children of modules into scope. That’s why we still have to use of::nested_modules when we want to call the nested_modules function.

Because enums also form a sort of namespace like modules, we can bring an enum’s variants into scope with use as well. For any kind of use statement, if you’re bringing multiple items from one namespace into scope, you can list them using curly brackets and commas in the last position

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow}; // or use TrafficLight::*

```

To traverse module hierarchy, use ```::foo::bar``` to start from the root or ```super::foo::bar``` to start from the immediate parent
