// 4-19
// traits

// ある型が提供しなければならない機能をRustのコンパイラに伝える言語機能
fn mode001() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    // 型のシグネチャのみを定義
    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }
}

// trait境界
fn mode002() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    // 型のシグネチャのみを定義
    trait HasArea {
        fn area(&self) -> f64;
    }

    impl HasArea for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }
    // これは任意の型Tにareaが無いとダメになるので実際はダメ
//    fn print_area<T>(shape: T) {
//        println!("This shape has an area of {}", shape.area());
//    }

    // HasAreaを実装するあらゆる型Tなので、だいぶ限定される(OK)
    // したがってこれはよい
    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }
}

fn mode003() {
    struct Rectangle<T> {
        x: T,
        y: T,
        width: T,
        height: T,
    }
    // PartialEq がある型Tのみ許可
    impl<T: PartialEq> Rectangle<T> {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }
    let mut r = Rectangle {
        x: 0,
        y: 0,
        width: 47,
        height: 47,
    };
    assert!(r.is_square());

    r.height = 42;
    assert!(!r.is_square());
}

// できるけどダメ
fn mode004() {
    trait HasArea {
        fn area(&self) -> f64;
    }
    impl HasArea for i32 {
        fn area(&self) -> f64 {
            println!("this is silly");

            *self as f64
        }
    }

    5.area();
}


// useを外すとできない
// -> useしないとWriteトレイトを使用できない
fn mode005() {
    use std::io::Write;

    let mut f = std::fs::File::open("foo.txt").expect("Couldn’t open foo.txt");
    let buf = b"whatever"; // buf: &[u8; 8] はバイト文字列リテラルです。
    let result = f.write(buf);
}

fn mode006() {
    use std::fmt::Debug;

    // 複数指定もできる
    fn foo<T: Clone + Debug>(x: T) {
        x.clone();
        println!("{:?}", x);
    }

    let a = vec![1, 2, 3, 4, 5];
    foo(a);
}

// where で最後にきじゅつできる(すっきり)
fn mode007() {
    use std::fmt::Debug;

    fn foo<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
        x.clone();
        y.clone();
        println!("{:?}", y);
    }
    foo("Hello", "world");
}


fn mode008() {
    trait ConvertTo<OutPut> {
        fn convert(&self) -> OutPut;
    }
    impl ConvertTo<i64> for i32 {
        fn convert(&self) -> i64 { *self as i64 }
    }
    // T:i32のとき
    fn normal<T: ConvertTo<i64>>(x: &T) -> i64 {
        x.convert()
    }

    fn inverse<T>() -> T where i32: ConvertTo<T> {
        43.convert()
    }
}


// default method
fn mode009() {
    trait Foo {
        fn is_valid(&self) -> bool;

        fn is_invalid(&self) -> bool { !self.is_valid() }
    }


}

fn mode010(){
    trait Foo {
        fn foo(&self);
    }
    trait FooBar : Foo {
        fn foobar(&self);
    }
    struct Baz;

    impl Foo for Baz {
        fn foo(&self) { println!("foo"); }
    }

    impl FooBar for Baz {
        fn foobar(&self) { println!("foobar"); }
    }
}

// Derive
fn mode011(){
    #[derive(Debug)]
    struct Foo;
    println!("{:?}",Foo);

}

fn main() {
    mode011();
}