// 4-29
// casting-between-types

// as 安全キャスト
fn mode001(){
    let x:i32 = 5;
    let y = x as i64;
}

// 数値キャスト
// 切り捨てやゼロ拡張などがある　普通
fn mode002(){
    let one = true as u8;
    let at_sign = 64 as char;
    let two_hundred = -56i8 as u8;
}

// :thinking_face:
fn mode003(){
    use std::mem;

    unsafe {
        let a = [0u8, 0u8, 0u8, 0u8];

        let b = mem::transmute::<[u8; 4], u32>(a);
    }
}


fn main(){
    mode003();
}