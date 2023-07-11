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

// トレイとを使うと、さまざまな型に共通のメソッドの実装ができる
trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("Uoooooohhhh!!!")
    }
}

struct Duck {}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("Quack!");
    }
}

struct Dove {}

impl Tweet for Dove {
    fn tweet(&self) {
        println!("Coo!")
    }
}

fn main() {
    let p = Person {
        name: String::from("Taro"),
        age: 25,
    };

    p.say_name();
    p.say_age();

    let duck = Duck {};
    duck.tweet();

    let dove = Dove {};
    dove.tweet();
    dove.shout();
}
