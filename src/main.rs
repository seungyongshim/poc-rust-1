fn main() {
   let a = goodbye("Shim");
   println!("{}", a);
}

fn goodbye(message: &str) -> u32 {
    println!("Goodbye. {}", message);
    32
}