// 4-3

// bool
fn mode001() {
    let x = true;
    let y: bool = true;
}

// char
fn mode002() {
    let x = 'z';
    let two_hearts = '💕';
    println!("alpha:{}, love:{}", x, two_hearts);
}

// number
// pass


// 配列
fn mode003() {
    let a = [1, 2, 3];
    let mut m = [1, 3, 4];
    println!("numbers of a: {:?}, m: {:?}", a, m);

    // [T;N] : Nはコンパイル時定数
    let a = [0; 20];
    println!("numbers of a: {:?}, m: {:?}", a, m);
    println!("a has {} elements", a.len());

    let names = ["Graydon", "Brian", "Niko"];
    println!("The second name is: {}", names[1]);
}

// Slice &と[]をどちらも使用する
fn mode004() {
    let a = [0, 1, 2, 3, 4];
    let complete = &a[..];
    let middle = &a[1..4];
    println!("complete of a: {:?}", complete);
    println!("middle of a: {:?}", middle);


    // compile error
//    let b = a[2..];
//    println!("b of a: {:?}",b);
}

fn mode005() {
    let x = (1, "hello!");
    let y: (i32, &str) = (2, "world!");

    let mut a = (1, 2);
    let b = (2, 5);
    a = b;

    let (c, d) = x;
    println!("c,d is:{},{}", c, d);

    // 一要素のtupleは、(a,)のように記述しなければならないため、次はcompile errorになる
//    let r = 10000;
//    let (rr) = r;
//    println!("rr is: {}",rr);
}

// tuple
fn mode006() {
    let tuple = (1, 3, 4);
    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x,y,z: {},{},{}", x, y, z);
}


fn main() {
    mode006();
}