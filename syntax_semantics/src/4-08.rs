// 4-8
// references and borrowing


// 借用
fn mode001() {
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for val in v1 {
            sum += val;
        }
        for val in v2 {
            sum += val;
        }
        sum
    }

    let v1 = vec![1, 2, 4];
    let v2 = vec![1000, 2000];
    let answer = foo(&v1, &v2);
    println!("ans is :{}", answer);

    // ここで所有権が失われていない
    println!("vector : {:?}", v1);
    println!("vector : {:?}", v2);
}

// 参照ではダメ
//fn mode002(){
//    fn foo(v:&Vec<i32>){
//        v.push(5);
//    }
//    let v =vec![];
//
//    foo(&v);
//}


// &mut 参照
fn mode003() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("{}", x);
}

// Rule : 参照と、mutable参照は同時に持つことができない(嬉しい)

// scope 借用を終了してくれない
//fn mode004(){
//    let mut x = 5;
//    {
//        let y = &mut x;
//        *y +=1;
//        println!("{}",x);
//    }
//}

// 同様にダメ
//fn mode005() {
//    let mut x = 5;
//    let y = &mut x;
//    *y += 1;
//    println!("{}", x);
//}


// 解放後の使用は禁止
// 借用変数より後まで生存していないといけない
//fn mode006() {
//    let y: &i32;
//    {
//        let x = 5;
//        y = &x;
//    }
//    println!("y is {}", y);
//}

// yを先に定義すると解放はあとになるので、後で定義すればこの場合はOK!
fn mode007() {
    let x = 5;
    let y: &i32;
    y = &x;
    println!("y is {}", y);
}


fn main() {
    mode007();
}