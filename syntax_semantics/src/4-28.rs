// 4-28
// type alias


fn mode001(){
    type Name = String ;
    let x : Name = "Hello".to_string();

    // 完全にaliasなので、
    let y = "Hello".to_string();

    if x  == y {
        println!("same type"); // こっちになる
    }else {
        println!("not same type");
    }

    // 結局、完全に異なる型がほしいときはtuple構造体を使う
}

// alias + generics
fn mode002(){
    use std::result;
    enum ConcreteError{
        Foo,
        Bar,
    }

    type Result<T> = result::Result<T,ConcreteError>;
}

fn main(){
    mode002();
}