// 4-25
// crates and modules

extern crate syntax_semantics;

use syntax_semantics::english::greetings;
use syntax_semantics::english::farewells::goodbye;

fn mode001(){

    // 関数内はダメらしい(そう…)
//    use syntax_semantics::english::greetings;
//    use syntax_semantics::english::farewells::goodbye;


    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", goodbye());

    //println!("Hello in Japanese: {}", syntax_semantics::japanese::greetings::hello());
    //println!("Goodbye in Japanese: {}", syntax_semantics::japanese::farewells::goodbye());
}


fn mode002(){
    // rename など、選択的、補助的なuse文がある
}

fn main() {
    mode001();
}