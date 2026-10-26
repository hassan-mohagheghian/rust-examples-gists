//! # Rust Struct Update Syntax and Partial Moves
//!
//! This example demonstrates how struct update syntax (`..user1`) works.
//!
//! ## Key rules
//! - `Copy` fields, such as `bool`, are copied.
//! - Non-`Copy` fields, such as `String`, are moved.
//! - Fields explicitly replaced in the new struct remain available
//!   on the original struct.
//! - After a partial move, unmoved fields remain accessible, but
//!   the original struct cannot be used as a whole.

struct User {
    active: bool,     // Copy
    username: String, // Move
    email: String,    // Move
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Alice"),
        email: String::from("alice@example.com"),
    };

    // `active` is copied.
    // `username` is moved.
    // `email` is replaced with a new value.
    let user2 = User {
        email: String::from("new@example.com"),
        ..user1
    };

    // `active` was copied, so it remains accessible.
    println!("user1.active: {}", user1.active);

    // `email` was replaced, not moved from user1.
    println!("user1.email: {}", user1.email);

    // `username` was moved into user2.
    // println!("{}", user1.username); // Compile-time error

    // All fields are accessible on user2.
    println!("user2.active: {}", user2.active);
    println!("user2.username: {}", user2.username);
    println!("user2.email: {}", user2.email);

    // `user1` is partially moved, so the whole struct
    // cannot be used.
    // println!("{:?}", user1); // Error: User is not Debug,
    // and user1 is partially moved.
}
