fn main() {
    let s1 = "hello"; // immutable string literal stored on the stack

    let mut s = String::from("hello"); // mutable string stored on the heap (the heap is what allows it to be mutable)
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);
}
