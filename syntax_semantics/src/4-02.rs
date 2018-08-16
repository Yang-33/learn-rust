// 4-2 functions

fn mode001() {
    fn print_number(x: i32) {
        println!("x is: {}", x);
    }
    print_number(5);
}

fn mode002() {
    fn print_sum(x: i32, y: i32) {
        println!("sum is : {}", x + y);
    }
    print_sum(10, 33);
}

// 型推論があっても引数は明示しなければならない
//fn mode003(){
//    fn print_sum(x,y){
//        println!("sum is :{}",x,y);
//    }
//    print_sum(10,303);
//}

// return val is not unit
fn mode004() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    println!("add one! :{}", add_one(100))
}

// return val is not unit -> compile error になる
//fn mode005(){
//    fn add_one(x:i32){
//        x+1
//    }
//    println!("add one! :{}",add_one(100))
//}

// 早期return
fn mode006() {
    fn foo(x: i32) -> i32 {
        return x;
        x + 1
    }
    println!("早期return! {}", foo(100));
}

// 発散する関数は !型(diverges)
// RUST_BACKTRACE=1 cargo runで情報を見ることができる
fn mode007() {
    fn diverges() -> ! {
        panic!("この関数はreturnしません!");
    }
    diverges();
}

// 発散する関数は任意の型としても使える
fn mode008() {
    fn diverges() -> ! {
        panic!("この関数はreturnしません!");
    }
    let x: i32 = diverges();
    let x: String = diverges();
}

// 関数ポインタ 明示
fn mode009() {
    fn plus_one(i: i32) -> i32 {
        i + 1
    }

    let f: fn(i32) -> i32 = plus_one;

    let six = f(5);
    println!("res is {}", six);
}

// 関数ポインタ 明示しない (しなくても関数定義からできる)
fn mode010() {
    fn plus_one(i: i32) -> i32 {
        i + 1
    }

    let f = plus_one;

    let six = f(5);
    println!("res is {}", six);
}


fn main() {
    mode010();
}
