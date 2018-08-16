fn mode001() {
    let x: i32;
    println!("hello.");
}

//fn mode002(){ // need to initialize x.
//    let x: i32;
//    println!("The value of x is: {}",x);
//}

fn mode003() { // initialize
    let x: i32 = 10;
    println!("The value of x is: {}", x);
}

//fn mode004(){// scope
//    let x:i32 = 17;
//    {
//        let y:i32=3;
//        println!("The value of x is {} and value of y is {}.",x,y);
//    }
//    println!("The value of x is {} ans value of y is {}.",x,y);
//}

fn mode005() {// scope
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}.", x, y);
    }
}

fn mode006() { // shadowing
    let x: i32 = 8;
    {
        println!("{}", x);
        let x = 14;
        println!("{}", x);
    }
    println!("{}", x);
    let x = 42;
    println!("{}", x);
}

fn main() {
    mode006();
}
