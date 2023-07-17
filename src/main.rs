use std::thread;
use std::sync::{Arc, Mutex, mpsc};

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
   
    let mut handles = Vec::new();

    // for x in 0..10 {
    //     handles.push(thread::spawn(move || {
    //         println!("Hello world in thred: {}", x);
    //     }))
    // }

    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            // lockを使ってdataへの可変参照を得る
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    
    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);

    message_passing();
}

// fn calc_data(data: String) -> String {
//     println!("{}", data);
//     data
// }

fn message_passing() {
  let mut handles = Vec::new();
  let mut data = vec![1; 10];
  let mut snd_channels = Vec::new();
  let mut rcv_channels = Vec::new();

  for _ in 0..10 {
      // mainから各スレッドへのチャンネル
      let (snd_tx, snd_rx) = mpsc::channel();
      // 各スレッドからmainへのチャンネル
      let (rcv_tx, rcv_rx) = mpsc::channel();

      snd_channels.push(snd_tx);
      rcv_channels.push(rcv_rx);

      handles.push(thread::spawn(move || {
          let mut data = snd_rx.recv().unwrap();
          data += 1;
          let _ = rcv_tx.send(data);
      }));
  }

  // 各スレッドにdataの値を送信
  for x in 0..10 {
      let _ = snd_channels[x].send(data[x]);
  }

  // 各スレッドからの結果をdataに格納
  for x in 0..10 {
      data[x] = rcv_channels[x].recv().unwrap();
  }
  
  for handle in handles {
      let _ = handle.join();
  }

  dbg!(data);
}
