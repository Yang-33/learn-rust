// 4-16
// vector

fn mode001() {
    let v = vec![1, 2, 3, 4, 5];

    let v = vec![1; 10];
    println!("vec is {:?}", v);
}

fn mode002() {
    let v = vec![1, 2, 3, 4, 5];
    println!("Thr third element of v is {}", v[2]);
}

// indexはusizeでないとダメ
fn mode003() {
    let v = vec![1, 2, 3, 4, 5];
    let i = 0;
    v[i];
    let j: i32 = 0;
//    v[j];
}

// 適当なiteratorを使うことができる。
fn mode004() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }

    for i in &mut v {
        println!("A mutable reference to {}", i);
        *i+=2;
    }

    for i in v{
        println!("Take ownership of the vector and its element {}",i);
    }

}

fn main() {
    mode004();
}