// 4-6
// loop


// 無限ループ
fn mode001() {
    loop {
        println!("Loop forever!");
    }
}

// while
fn mode002() {
    let mut x = 5;
    let mut done = false;
    while !done {
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            done = true;
        }
    }
}

// 安全性が低い(loop)とは等価に扱われない
fn mode003() {
    while true {
        println!("Loop forever!");
    }
}

// for

fn mode004() {
    for x in 0..10 {
        println!("x is {}", x);
    }
}

// enumerate
fn mode005() {
    for (i, j) in (5..10).enumerate() {
        println!("i = {}, j = {}", i, j);
    }
}

// Lines型というものがある
// enumerateは&strにはなかった
fn mode006() {
    let lines = "hello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }
}

// 反復の早期終了
fn mode007() {
    let mut x = 5;

    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            break;
        }
    }
}

fn mode008() {
    for x in 00..20 {
        if x % 2 == 0 { continue; }
        println!("{}", x);
    }
}

// loop label
// 競技的には便利そうだけど一般的にはそこまで便利ではなさそう
fn mode009() {
    'outer:
        for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x:{}, y:{}", x, y);
        }
    }
}

fn main() {
    mode009();
}
