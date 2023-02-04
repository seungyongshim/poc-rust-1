fn main() {
   let a = goodbye("Shim");
   println!("{}", a);
}

fn goodbye(message: &str) -> u32 {
    println!("Goodbye. {}", message);
    32
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod add_function_tests {
    use super::add;

    #[test]
    fn add_works() {
        assert_eq!(add(1,2), 3);
    }
}