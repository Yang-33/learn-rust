// 4-23
// Closure


fn mode001() {
    let plus_one = |x: i32| x + 1;
    println!("{}", plus_one(10));
}

fn mode002() {
    let plus_two = |x| {
        let mut result: i32 = x;
        result += 1;
        result += 1;
        result
    };
    println!("{}", plus_two(100));
}

// 関数と違って、引数や返り値の型を示す必要がない(書くことはできる)
fn mode003() {
    let plus_one = |x: i32| -> i32{ x + 1 };
    println!("{}", plus_one(10));
}


// closureのscope [&] っぽい
fn mode004() {
    let num = 5;
    let plus_num = |x| x + num;
    println!("{}", plus_num(1000));
}

// closureの借用
// これはダメ
fn mode005() {
//    let mut num = 5;
//    let plus_num = |x| x + num;
//    println!("{}", plus_num(100));
//    let y = &mut num;
}

// これはOK
fn mode006() {
    let mut num = 5;
    {
        let plus_num = |x| x + num;
        println!("{}", plus_num(100));
    }
    let y = &mut num;
}


// move closureとの比較
fn mode007() {
    let mut num = 5;

    {
        let mut add_num = |x: i32| num += x;

        add_num(5);
    }

    assert_eq!(10, num);
}

// moveではcopyしてくる
fn mode008() {
    let mut num = 5;

    {
        let mut add_num = move |x: i32| num += x;

        add_num(5);
    }

    assert_eq!(5, num);
}


// closureの実装
fn mode009() {
//    mod foo {
//        pub trait Fn<Args>: FnMut<Args> {
//            extern "rust-call" fn call(&self, args: Args) -> Self::Output;
//        }
//
//        pub trait FnMut<Args>: FnOnce<Args> {
//            extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
//        }
//
//        pub trait FnOnce<Args> {
//            type Output;
//
//            extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
//        }
//    }
}

fn mode010(){
    fn call_with_one<F>(some_closure:F) -> i32
    where F: Fn(i32)->i32{
        some_closure(1)
    }
    let answer = call_with_one(|x| x+2);
    println!("answer: {}",answer);

}

// closureの参照(?)
fn mode011(){
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }

    let answer = call_with_one(&|x| x + 2);

    assert_eq!(3, answer);
}

// これもできる(というかできてほしい)
fn mode012(){
    fn call_with_one(some_closure: &Fn(i32) -> i32) -> i32 {
        some_closure(1)
    }

    fn add_one(i: i32) -> i32 {
        i + 1
    }

    let answer = call_with_one(&add_one);

    assert_eq!(2, answer);
}


// 以下関数を返したい

// 返却のサイズが分からないのでダメ
fn mode013(){
//    fn factory() -> (Fn(i32) -> i32) {
//        let num = 5;
//
//        |x| x + num
//    }
//
//    let f = factory();
//
//    let answer = f(1);
//    assert_eq!(6, answer);
}

// 参照ならサイズは分かる
// でもダメで、lifetimeが必要そう

fn mode014(){
//    fn factory() -> &(Fn(i32) -> i32) {
//        let num = 5;
//
//        |x| x + num
//    }
//
//    let f = factory();
//
//    let answer = f(1);
//    assert_eq!(6, answer);
}

// staticにしてみた
// 'static lifetime をobjectに割り当てることはできない
fn mode015(){
//    fn factory() -> &'static (Fn(i32) -> i32) {
//        let num = 5;
//
//        |x| x + num
//    }
//
//    let f = factory();
//
//    let answer = f(1);
//    assert_eq!(6, answer);
}

// Fn をbox化すればトレイとオブジェクトを返すことができる
// でもnumの生存期間と返却される関数のlifetimeが異なる
fn mode016(){
//    fn factory() -> Box<Fn(i32) -> i32> {
//        let num = 5;
//
//        Box::new(|x| x + num)
//    }
//    let f = factory();
//
//    let answer = f(1);
//    assert_eq!(6, answer);
}

// 解決
// 現在のstack flameから抜ければよい
fn mode017(){
    fn factory() -> Box<Fn(i32) -> i32> {
        let num = 5;

        Box::new(move |x| x + num)
    }
    let f = factory();

    let answer = f(1);
    assert_eq!(6, answer);
}

fn main() {
    mode017();
}