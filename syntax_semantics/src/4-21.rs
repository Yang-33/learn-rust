// 4-21
// if let


fn mode001() {
    let option = Some(5);
    fn foo(x: i32) {
        println!("in foo, x: {}", x);
    }
    // match
    match option {
        Some(x) => { foo(x); }
        None => {}
    }

    // match なしでもかける
    if option.is_some() {
        let x = option.unwrap();
        foo(x);
    }

    // 理想的
    // パターンマッチに成功した場合にlet xに値が割り当てられる
    // else も書ける
    // ちょっと気持ち悪いけどmatchをかんがえればrust的には普通
    if let Some(x) = option{
        foo(x);
    }else{
        // ..
    }
}


// while let
fn mode002(){
    let mut v = vec![1,3,5,7,11];
    loop{
        match v.pop() {
            Some(x) => println!("{}",x),
            None => break,
        }
    }
}

// loop match ... をwhile let ...にできる
fn mode003(){
    let mut v = vec![1,3,5,7,11];
    while let Some(x) = v.pop(){
        println!("{}",x);
    }
}

fn main() {
    mode003();
}