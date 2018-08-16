// 4-20
// Drop

// Drop traitは値がスコープ外になったときにコードを実行する方法を提供する

// デストラクタっぽい
fn mode001() {
    // 名前を変えるとダメなので、Drop dropでないとだめ
    struct HasDrop;
    impl Drop for HasDrop {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }
    let x = HasDrop;
}


// 解放は定義された順にstackに積まれていることに注意(借用らへんでもやった)
fn mode002() {
    struct Firework {
        strength: i32,
    }
    impl Drop for Firework {
        fn drop(&mut self) {
            println!("BOOM times {}!!!", self.strength);
        }
    }
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };
}


fn main() {
    mode003();
}