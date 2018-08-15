// 4-11
// struct


fn mode001() {
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    println!("The origin is at {} {}", origin.x, origin.y);
}


fn mode002() {
    struct Point {
        x: i32,
        y: i32,
    }
    let mut point = Point { x: 0, y: 0 };
    println!("The point is at ({}, {})", point.x, point.y);
    point.x = 10;
    println!("The point is at ({}, {})", point.x, point.y);
}

fn mode003() {
    struct Point {
        x: i32,
        y: i32,
    }
    let mut point = Point { x: 0, y: 0 };
    println!("The point is at ({}, {})", point.x, point.y);
    point.x = 10;
    println!("The point is at ({}, {})", point.x, point.y);

    let point = point;
    println!("The point is at ({}, {})", point.x, point.y);

    // 当然immutableなのでダメ
//    point.y = 100;
    println!("The point is at ({}, {})", point.x, point.y);
}

// update構文
fn mode004() {
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }
    let mut point = Point3d { x: 0, y: 0, z: 0 };

    // ここから
    point = Point3d { y: 1, ..point };
}

// update構文2
fn mode005() {
    struct Point3d {
        x: i32,
        y: i32,
        z: i32,
    }
    // immutable
    let origin = Point3d { x: 0, y: 0, z: 0 };
    // こういうこともできちゃう
    let point = Point3d { z: 1, x: 2, ..origin };
}

// tuple structure
// 1変数でも嬉しいことになる
fn mode006() {
    struct Inches(i32);

    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);
}

// Unit Like
fn mode007() {
    struct A;
    let x = A;
}


fn main() {
    mode007();
}