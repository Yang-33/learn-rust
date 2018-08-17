// 4-22
// trait objects


fn mode001() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8:{}", *self) }
    }
    impl Foo for String {
        fn method(&self) -> String { format!("string:{}", *self) }
    }
}

fn mode002() {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8:{}", *self) }
    }
    impl Foo for String {
        fn method(&self) -> String { format!("string:{}", *self) }
    }
    fn do_something<T: Foo>(x: T) {
        x.method();
    }
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(x);
    do_something(y);
}

// 動的ディスパッチ
fn mode003() {
    trait Foo { fn method(&self) -> String; }
    impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
    impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }

    fn do_something(x: &Foo) {
        x.method();
    }

    let x = 5u8;
    do_something(&x as &Foo);
}




fn main() {
    mode003();
}