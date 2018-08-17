// 4-33
// Derefによる型強制



// Deref は参照外し演算子 * のオーバーロードに用いられる
fn mode001(){
    use std::ops::Deref;
    struct DerefExample<T>{
        value:T,
    }
    impl<T> Deref for DerefExample<T>{
        type Target = T;
        fn deref(&self) -> &T{
            &self.value
        }
    }
    let x = DerefExample{value:'a'};
    assert_eq!('a',*x);
}

fn mode002(){
    fn foo(s: &[i32]) {
        // スライスを一瞬だけ借用します
    }

// Vec<T> は Deref<Target=[T]> を実装しています
    let owned = vec![1, 2, 3];

    foo(&owned);
}


// なるほどなあ　ああ　なるほどなあ
fn mode003(){
    struct Foo;

    impl Foo {
        fn foo(&self) { println!("Foo"); }
    }

    let f = &&Foo;

    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}

fn main(){
    mode003();
}