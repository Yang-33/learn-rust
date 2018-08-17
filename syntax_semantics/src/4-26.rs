// 4-26
// const & static


fn mode001(){
    static N:i32 = 5;
    // 型を明示しないといけない

    static NAME: &'static str = "Steve";
    // 'staticでなくとも全体に生存期間を持つ
}

// static mut のrule
fn mode002(){
    static mut N :i32= 5;

    // そのままはできない
//    N+=1;

    // unsafe blockで囲う
    unsafe{
        N+=1;
        println!("N: {}",N);
    }

}

fn main(){
    mode002();
}