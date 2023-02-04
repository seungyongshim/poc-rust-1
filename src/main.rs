fn main() {
    let tuple = ('E', 5i32, true);

    println!("{}", tuple.0);

    let user_1 = Student
    {
        name: String::from("Shim"),
        level: 2,
        remote: true
    };

    println!("{}", user_1.name);
}


struct Student
{
    name : String,
    level: u8,
    remote: bool
}