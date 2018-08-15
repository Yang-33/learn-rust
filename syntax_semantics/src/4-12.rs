// 4-12
// enums

// 識別子は衝突しない
fn mode001(){
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }
    let x:Message = Message::Move{x:3,y:10};
    enum BoardGameTurn {
        Move{squares: i32},
        Pass,
    }
    let y: BoardGameTurn = BoardGameTurn::Move{squares:1};
}


fn mode002(){
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    // この形式はできないので、等値性を実装するかmatchで解決する
//    fn process_color_change(msg: Message) {
//        let Message::ChangeColor(r, g, b) = msg;
//    }
}

// 列挙型のコンストラクタを関数のように使う
fn mode003(){
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }

    let m = Message::Write("Hello, world".to_string());

    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1:Vec<Message> = v.into_iter().map(Message::Write).collect();
}

fn main() {
    mode003();
}

