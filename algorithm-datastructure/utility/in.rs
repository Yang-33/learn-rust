#[allow(dead_code)]
fn read_line() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    ret.pop();
    return ret;
}

#[allow(dead_code)]
fn read_i64() -> i64 {
    let ss = read_line();
    return ss.parse::<i64>().unwrap();
}

#[allow(dead_code)]
fn read_vec_i64() -> Vec<i64> {
    let mut res = vec![];
    let ss = read_line();
    for ts in ss.split_whitespace() {
        let x = ts.parse::<i64>().unwrap();
        res.push(x);
    }
    return res;
}

#[allow(unused_macros)]
macro_rules! from_line {
    ($($a:ident : $t:ty),+) => {
        $(let $a: $t;)+
        {
            let _line = read_line();
            let mut _it = _line.trim().split_whitespace();
            $($a = _it.next().unwrap().parse().unwrap();)+
            assert!(_it.next().is_none());
        }
    };
}