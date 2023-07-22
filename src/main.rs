use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32);

impl Future for CountDown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready("Zero!!".to_string())
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

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

struct Color {
    r: i32,
    g: i32,
    b: i32,
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

    let a = Color {
        r: 255,
        g: 255,
        b: 255,
    };
    let b = a; //所有権がbに譲渡される

    println!("{}, {}, {}", b.r, b.g, b.b);

    // 借用 = 参照
    // 関数の引数で値を渡す時、値の所有権ごと渡してしまうと、呼び出し元の処理に再び所有権を返すのは面倒なので、
    // 値を関数に渡す時は、値の参照を渡すようにする
    // 参照 = 所有権を別の変数に譲渡するのではなく、値へのアクセスを許す方法。所有権は変わらない
    // 参照は元の所有者のライフタイムよりも長く生きることはできない

    // let mut important_data = "Hello, World!!".to_string();

    // important_data = calc_data(important_data);

    // println!("{}", important_data);

    rust_programming_for_beginner::module_a::module_fn();
}

// fn calc_data(data: String) -> String {
//     println!("{}", data);
//     data
// }
