struct Person {
    name: String,
    age: u32,
}

// implで、構造体にメソッドを加えることができる
impl Person {
    fn say_name(&self) {
        println!("I am {}.", self.name);
    }

    fn say_age(&self) {
        println!("I am {} years old.", self.age);
    }
}

fn main() {
    let p = Person {
        name: String::from("Taro"),
        age: 25,
    };

    p.say_name();
    p.say_age();
}
