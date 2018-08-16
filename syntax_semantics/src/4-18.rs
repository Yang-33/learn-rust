// 4-18
// generics

fn mode001() {
    let x: Option<i32> = Some(5);
}

// 型不一致はダメ
fn mode002() {
//    let x: Option<f64> = Some(5);
}

// generic function
fn mode003() {
    fn takes_anything<T>(x: T) {
        //
    }
    fn takes_two_of_the_same_things<T>(x: T, y: T) {
        //
    }
    fn takes_two_things<T, U>(x: T, y: U) {
        //
    }
}


// 当然構造体もできる
fn mode004() {
    struct Point<T> {
        x: T,
        y: T,
    }
    // 型指定しなくても推論してくれる
    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };

    impl<T> Point<T>{
        fn swap(&mut self){
            std::mem::swap(&mut self.x, &mut self.y);
        }
    }
}




fn main() {
    mode004();
}