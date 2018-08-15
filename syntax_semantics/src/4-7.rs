// 4-7
// ownership

fn mode001() {
    let v = vec![1, 2, 3];
}

// 所有権がかわったのでダメ(メモリを指すもののみ変化し、ヒープ上は変化していない
//fn mode002() {
//    let v = vec![1, 2, 3];
//    let v2 = v; //move
//    println!("v[0] is {}", v[0]);
//}

fn mode003() {
    fn take(v: Vec<i32>) {
        println!("{:?}", v);
    }
    let v = vec![1, 2, 3];
    take(v);

    // 所有権がかわったのでこれもダメ
//    println!("after move, v is {:?}(error)", v);
}


fn mode004() {
    fn take(v: &Vec<i32>) {
        println!("{:?}", v);
    }
    let v = vec![1, 2, 3];
    take(&v);

    // 参照なら大丈夫
    println!("after move, v is {:?}(--error--)", v);
}

// type is Copy
fn mode005() {
    let v = 1;
    let v2 = v;
    println!("v is :{}", v);
}

// Copy Traitsがあるものだけは所有権などを考えなくてもよい(メモリを理解すればそれはそう)
fn mode006() {
    fn double(x: i32) -> i32 {
        x * 2
    }

    let a = 5;
    let _y = double(a);
    println!("{}", a);
}

// 借用なしで所有権を返す(気合)
// ひどい感じになる
fn mode007(){
    fn foo(v1:Vec<i32>, v2:Vec<i32>)->(Vec<i32>,Vec<i32>,i32){
        println!("v1 is {:?} on foo",v1);
        println!("v2 is {:?} on foo",v2);
        (v1,v2,4000)
    }

    let v1 = vec![1,2,3,4];
    let v2=vec![3,54,3,43];

    let (v1,v2,answer) = foo(v1,v2);
    println!("v1 is {:?} on main",v1);
    println!("v2 is {:?} on main",v2);
}

fn main() {
    mode007();
}