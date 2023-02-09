struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // println!("{}", user1.username); // ! errors b/c it was moved to user2.username and it is heap-only data (no Copy trait)
    println!("{}", user1.email); // ! does not error bc it was not moved
}
