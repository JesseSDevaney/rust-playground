#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1); // * :? inside {} tells rust to print out debugging information
    println!("rect1 is {:#?}", rect1); // * prettier output for structs
}
