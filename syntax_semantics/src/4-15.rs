// 4-15
// method syntax


// method呼び出し
fn mode001() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }
    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());
}

fn mode002() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    // &self を使用すべきではあるが
    impl Circle {
        fn reference(&self) {
            println!("taking self by reference");
        }

        fn mutable_reference(&mut self) {
            println!("taking self by mutable reference");
        }

        fn takes_ownership(self) {
            println!("taking ownership of self");
        }
    }
}


fn mode003() {
    // 他言語同様にimplを分けて良い
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn reference(&self) {
            println!("taking self by reference!");
        }
    }

    impl Circle {
        fn mutable_reference(&mut self) {
            println!("taking self by mutable reference!");
        }
    }

    impl Circle {
        fn takes_ownership(self) {
            println!("taking ownership of self!");
        }
    }
}

// method chain
fn mode004() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        fn grow(&self, increment: f64) -> Circle {
            Circle { x: self.x, y: self.y, radius: self.radius + increment }
        }
    }

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area());

    let d = c.grow(2.0).area();
    println!("{}", d);
}

// 関連関数
fn mode005() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: x,
                y: y,
                radius: radius,
            }
        }
    }
    let c = Circle::new(0.0, 0.0, 2.0);
}


// Builder パターン
// 可変個引数、名前付き引数、オーバーロードは存在しないが
// 実際にはこれで大体のしたいことはできる
fn mode006() {
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    struct CircleBuilder {
        x: f64,
        y: f64,
        radius: f64,
    }

    impl CircleBuilder {
        fn new() -> CircleBuilder {
            CircleBuilder { x: 0.0, y: 0.0, radius: 2.0 }
        }
        fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
            self.x = coordinate;
            self
        }
        fn y(&mut self, coordinate: f64)-> &mut CircleBuilder{
            self.y = coordinate;
            self
        }
        fn radius(&mut self, radius: f64) -> &mut CircleBuilder{
            self.radius = radius;
            self
        }
        fn finalize(&self)-> Circle{
            Circle{x:self.x, y:self.y, radius:self.radius}
        }
    }

    let c = CircleBuilder::new()
        .x(1.0).y(1.0).radius(2.0).finalize();

    println!("area: {}", c.area());

    println!("x:{}", c.x);
    println!("y:{}", c.y);


}


fn main() {
    mode006();
}