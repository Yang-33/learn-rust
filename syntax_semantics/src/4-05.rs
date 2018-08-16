// 4-5
// if

fn mode001() {
    let x = 5;
    if x == 5 {
        println!("x is 5!");
    }
}

fn mode002() {
    let x = 3;
    if x == 5 {
        println!("x is 5!");
    }
}

fn mode003() {
    let x = 5;
    if x == 5 {
        println!("x is 5!");
    } else {
        println!("x is not 5!");
    }
}

fn mode004() {
    let x = 4;
    if x == 5 {
        println!("x is 5!");
    } else {
        println!("x is not 5!");
    }
}

fn mode005() {
    let x = 5;
    if x == 5 {
        println!("x is 5!");
    } else if x == 6 {
        println!("x is not 6!");
    } else {
        println!("e----");
    }
}

fn mode006() {
    let x = 5;
    let y = if x == 5 {
        10
    } else {
        15
    };
    println!("y is: {}", y);
}

// better
fn mode007() {
    let x = 6;
    let y = if x == 6 { 100 } else { 3000 };
    println!("y is:{}",y);
}

fn main() {
    mode007();
}