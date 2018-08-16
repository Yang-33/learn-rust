// 4-14
// patterns


// シャドーイングがあるので注意すること
fn mode001() {
    let x = 'x';
    let c = 'c';

    match c {
        x => println!("x: {} c: {}", x, c),
    }

    println!("x: {}", x);
}


fn mode002() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    };
}

// 分配束縛(002もよくみておくこと)
fn mode003() {
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, y } => println!("({},{})", x, y),
    }
}

// 先はシャドーイングなので当然別名をつけることができる
fn mode004() {
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
    }
}

// 必要なものにだけ名前をつければよい
fn mode005() {
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 1 };
    match origin {
        Point { y: y1, .. } => println!("({})", y1),
    }
}

// 束縛の無視
fn mode006() {
    let some_value: Result<i32, &'static str> = Err("There was an error");
    match some_value {
        Ok(value) => println!("got a value {}", value),
        Err(_) => println!("an error occurred"),
    };
}

// 束縛の無視 ..で複数値の無視(005でも使用した)
fn mode007() {
    enum OptionalTuple {
        Value(i32, i32, i32),
        Missing,
    }
    let x = OptionalTuple::Value(5, -2, 3);

    match x {
        OptionalTuple::Value(..) => println!("Got a tuple!"),
        OptionalTuple::Missing => println!("No such luck"),
    }
}

// 参照にもできる
fn mode008() {
    let x = 5;

    match x {
        ref r => println!("Got a reference to {}", r),
    }
}


fn mode009() {
    let mut x = 5;

    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }
}

// ... で範囲のmatch (..ではない!)
fn mode010() {
    let x = 1;

    match x {
        1...5 => println!("one througn five"),
        _ => println!("anything"),
    }
}


fn mode011() {
    let x = '💅';

    match x {
        'a'...'j' => println!("early letter"),
        'k'...'z' => println!("late letter"),
        _ => println!("something"),
    };
}

// 束縛
fn mode012() {
    let x = 1;

    match x { // 範囲指定で束縛(今までの書き方だとシャドーイングのみだった
//        e => println!("got a range element {}",e),
        e @ 1...5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}

fn mode013() {
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }
    let name = "Steve".to_string();
    let mut x: Option<Person> = Some(Person { name: Some(name) });
    match x {
        Some(Person { name: ref a @ Some(_), .. }) => println!("{:?}", a),
        _ => {}
    }
}

fn mode014() {
    let x = 8;

    match x { // 書き方はこれしかダメ　それぞれのパターンで同じ名前が束縛されるようにする
//      e @ (1...5 | 8...10) => println!("got a range element {}",e),
        e @ 1...5 | e @ 8...10 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}


// ガード
fn mode015(){
    enum OptionalInt{
        Value(i32),
        Missing,
    }

    let x = OptionalInt::Value(7);

    match x { // ..でも_でもよいが、..だと複数パターン
        OptionalInt::Value(i) if i> 5  => println!("Got an int bigger than five!"),
        OptionalInt::Value(_) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck"),
    }
}

// 複式パターンでifを使うと、ifは両側に適用される
fn mode016(){
    let x = 4;
    let y = false;

    match x{
        4|5 if y => println!("yes"),
        _ => println!("no"),
    }

}

// 混合match
// いろいろできるね
fn mode017(){
    match x {
        Foo { x: Some(ref name), y: None } => ...
    }
}

fn main() {
    mode017();
}