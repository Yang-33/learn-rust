// 4-10
// mutability

use std::cell::RefCell;

fn mode001() {
    let mut x = 5;
    x = 6;
}

// immutable 参照
fn mode002() {
    let mut x = 5;
    let y = &mut x;
    println!("y is: {}", y);
//    println!("y is: {}",x);
}

fn mode003() {
    let mut x = 5;
    let mut y = &mut x;
}

// mut はパターンであることに留意
fn mode004() {
    let (mut x, y) = (5.6, 1010);
    println!("x, y = {}, {}", x, y);
    x = 1212.1;
    println!("x, y = {}, {}", x, y);
}


// Arc<T> はCopy Traitではない
//fn mode005(){
//    use std::sync::Arc;
//    let x = Arc::new(5);
//    let y = x;
//    println!("x,y = {}, {}",x,y);
//}

fn mode006() {
    use std::sync::Arc;
    let x = Arc::new(5);
    let y = x.clone();
    println!("x,y = {}, {}", x, y);
}

fn mode007() {
    use std::cell::RefCell;
    let x = RefCell::new(42);
    let y = x.borrow_mut();
}

// compileできるがダメなやつ
fn mode008() {
    use std::cell::RefCell;
    let x = RefCell::new(42);
    let y = x.borrow_mut();
    let z = x.borrow_mut();
}

// immutable and mutable は作れない(えー)
fn mode009(){
//    struct Point{
//        x:i32,
//        mut y:i32,
//    }
}

fn mode010(){
    struct Point{
        x:i32,
        y:i32,
    }
    let mut a = Point{x:9,y:5};
    a.x = 10;
    let b = Point{ x:5, y: 10};
    // これはダメ(それはそう…)
//    b.x = 1;
}

// こうすれば一応実現できる
// point.yはmutではないがCellは他の型をwrapするオブジェクト
fn mode011(){
    use std::cell::Cell;
    struct Point{
        x:i32,
        y:Cell<i32>,
    }
    let point = Point{x:5 , y: Cell::new(6) };
    println!("y:{:?}",point.y);
    point.y.set(100000);
    println!("y:{:?}",point.y);

}

fn main() {
    mode011();
}