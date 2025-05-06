fn main() {
    let s = String::from("Hello, world!");
    let p = &s as *const String; // Create a raw pointer to `s`
    println!("{}", *p); // Dereference the raw pointer to print `s`
    println!("{}", s);
}fn main() {
    unsafe {
        let s = String::from("Hello, world!");
        let p = &s as *const String; // Create a raw pointer to `s`
        println!("{}", *p); // Dereference the raw pointer to print `s`
        println!("{}", s); // Use `s` directly
    }
}