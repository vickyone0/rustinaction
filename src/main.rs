fn greet_world() {
    println!("Hello, world!");

    let south_germany = "Grüß Gott!";

    let japan = "こんにちは!";

    let region = [south_germany, japan];

    for greeting in region.iter() {
        println!("{}", greeting);
    }

}

fn main() {
    greet_world();
}