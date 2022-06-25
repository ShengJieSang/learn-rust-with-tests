pub mod hello;

fn main() {
    let mut name: String = "Chris".to_string();
    println!("{}", hello::hello(&mut name));
}
