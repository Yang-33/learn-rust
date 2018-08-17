// 4-24
// ufcs


// どっちかわからない
fn mode001() {
    trait Foo {
        fn f(&self);
    }

    trait Bar {
        fn f(&self);
    }

    struct Baz;

    impl Foo for Baz {
        fn f(&self) { println!("Baz’s impl of Foo"); }
    }

    impl Bar for Baz {
        fn f(&self) { println!("Baz’s impl of Bar"); }
    }

    let b = Baz;
//    b.f();
}

// 曖昧性の排除
fn mode002() {
    trait Foo {
        fn f(&self);
    }
    trait Bar {
        fn f(&self);
    }
    struct Baz;
    impl Foo for Baz {
        fn f(&self) { println!("Baz’s impl of Foo"); }
    }
    impl Bar for Baz {
        fn f(&self) { println!("Baz’s impl of Bar"); }
    }
    let b = Baz;
    Foo::f(&b);
    Bar::f(&b);
    // method でのb.f()は、f(&b)と等価なので明示的にやってやればよい
}

// <>::は型のヒント
fn mode003(){
    trait Foo {
        fn foo() -> i32;
    }

    struct Bar;

    impl Bar {
        fn foo() -> i32 {
            20
        }
    }

    impl Foo for Bar {
        fn foo() -> i32 {
            10
        }
    }
    assert_eq!(10, <Bar as Foo>::foo());
    assert_eq!(20, Bar::foo());
}

fn main() {
    mode003();
}