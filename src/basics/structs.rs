struct User {
    name: String,
    age: u32,
}

fn main() {
    let user_1: User = User{
        name: String::from("Alice"),
        age: 30,
    };

    println!("Hello, world! {} is {} years old.", user_1.name, user_1.age);
}
