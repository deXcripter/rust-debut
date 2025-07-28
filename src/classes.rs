// Classes (haha - structs really)

struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) -> String {
        format!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        )
    }

    fn have_birthday(&mut self) {
        self.age += 1;
        println!(
            "Happy birthday, {}! You are now {} years old.",
            self.name, self.age
        );
    }
}

pub fn manage_user() {
    let user = User::new("Johnpaul", 30);
    println!("{}", user.greet());

    let mut birthday_user = User::new("Johnpaul", 30);
    birthday_user.have_birthday();

    println!("{}", birthday_user.greet());
}
