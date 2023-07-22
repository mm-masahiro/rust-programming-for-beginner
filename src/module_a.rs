pub fn module_fn() {
    println!("test")
}

pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

// #[test]アトリビュートをつけたら、それ以降のものがcargo testで実行できる
// けど、これだとプロダクションのコードにテストコードが混ざっちゃうから、テストモジュールを作ってそっちでテスト書くとかのほうがいいかも
#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(1, 0));
    assert_eq!(1, add(0, 1));
    assert_eq!(2, add(1, 1));
    assert_ne!(3, add(1, 1));
}
