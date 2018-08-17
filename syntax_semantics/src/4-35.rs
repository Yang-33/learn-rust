// 4-35
// raw pointers


// 生ポインタ自体は安全
fn mode001(){
    let x = 5;
    let raw = &x as *const i32;
    let mut y = 10;
    let raw_mut = &mut y as *mut i32;
}

fn mode002(){
    let x = 5;
    let raw = &x as *const i32;
// これはダメ
//    println!("raw points at {}",*raw);
}

// どうしても使いたいときは、unsafe blockで解決してから使用する
fn mode003(){
    let x = 5;
    let raw = &x as *const i32;

    let points_at = unsafe { *raw };

    println!("raw points at {}", points_at);
}

// 参照と生ポインタ
fn mode004(){
    // 明示的なキャスト
    let i:u32 = 1;
    let p_imm: *const u32 = &i as *const u32;

    // 暗黙的なキャスト
    let mut m :u32 = 2;
    let p_mut : *mut u32 = &mut m;

    unsafe {
        let lef_imm:&u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;
    }

}

fn main(){
    mode004();
}