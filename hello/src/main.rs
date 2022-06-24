pub mod hello;

fn main() {
    let name: String = "Chris".to_string();
    println!("{}", hello::hello(&name));
}
