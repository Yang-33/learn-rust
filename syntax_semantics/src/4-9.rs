// 4-9
// lifetimes

// 参照の生存期間の明示
fn mode001() {
    struct Foo<'a> {
        x: &'a i32,
    }
    let y = &5;
    let f = Foo { x: y };
    println!("{}", f.x);
}

fn mode002() {
    struct Foo<'a> {
        x: &'a i32,
    }

    impl<'a> Foo<'a> {
        fn x(&self) -> &'a i32 { self.x }
    }

    let y = &5;
    let f = Foo { x: y };

    println!("x is: {}", f.x());
}

// 複数の生存期間(ライフタイム)
fn mode003() {
    // 適当に選択できる
//    fn x_or_y<'a>(x: &'a str, y: &'a str) -> &'a str {
//        y
//    }
//    fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//        x
//    }
}

// scope of lifetime
fn mode004() {
    struct Foo<'a> {
        x: &'a i32,
    }

    let y = &5;
    let f = Foo { x: y };
}

//fn mode005(){
//    struct Foo<'a> {
//        x: &'a i32,
//    }
//
//    let x ;
//    {
//        let y = &5;
//        let f = Foo{x:y};
//        x = &f.x; // ここでscope内を指すので解放されたものを指すことになり、これはダメ
//    }
//    println!("{}",x);
//}

// 'staticについて
fn mode006(){
    let x:&'static str = "hello world.";
    static FOO:i32 =5;
    let x:&'static i32 = &FOO;
}

fn main() {
    mode006();
}