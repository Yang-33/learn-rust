// 4-17
// string

fn mode001() {
    let greeting = "Hello there.";
    println!("{}", greeting);

    let s = "foo\
    bar";
    println!("{}", s);

    let s = "foo
    bar";
    println!("{}", s);

    let mut a = "Hello".to_string();
    println!("{}", a);

    a.push_str(", world");
    println!("{}", a);
}


fn mode002() {
    fn takes_slice(slice: &str) {
        println!("Got: {}", slice);
    }

    let s = "Hello".to_string();
    takes_slice(&s);
}

fn mode003() {
    use std::net::TcpStream;
    TcpStream::connect("192.168.0.1:3000");

    let addr_string = "192.168.0.1:3000".to_string();

    TcpStream::connect(&*addr_string);
}

// indexing
fn mode004() {
    let s = "hello";
    // indexingをサポートしてない
//    println!("The first letter of s is {}",s[0]);

    // unicode or point
    let hachiko = "忠犬ハチ公";
    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    for c in hachiko.chars() {
        print!("{}, ", c);
    }
    println!("");
}

// !
fn mode005() {
    let dog = "hachiko";
    let hachi = &dog[0..5];
    //  byteであってoffsetではないことに注意
    println!("{}", hachi);
    let dog = "忠犬ハチ公";
    let hachi = &dog[0..2];
    println!("{}", hachi);
}

// string の連結は、(string)^+ + &strのみ
fn mode006() {
    let hello = "Hello".to_string();
    let world  = "world!";

    let hello_world = hello + world;
}


fn main() {
    mode006();
}