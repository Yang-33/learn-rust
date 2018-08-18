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

/// verified 2018/08/18 http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A
struct UnionFind {
    par: Vec<Option<usize>>,
    sz: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind { par: vec![None; n], sz: vec![1; n] }
    }

    fn unionset(&mut self, x: usize, y: usize) {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry { return; }
        let (big, small) = if self.sz[rx] < self.sz[ry] { (ry, rx) } else { (rx, ry) };
        self.par[small] = Some(big);
        self.sz[big] += self.sz[small];
    }
    fn root(&mut self, x: usize) -> usize {
        match self.par[x] {
            Some(subx) => {
                let xtop = self.root(subx);
                self.par[x] = Some(xtop);
                xtop
            }
            None => {
                x
            }
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    fn size(&mut self, x: usize) -> i32 {
        let rt = self.root(x);
        self.sz[rt]
    }
}


fn main() {
    let onl = read_vec_i64();
    let (n, q) = (onl[0] as usize, onl[1] as usize);

    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        let line = read_vec_i64();
        let (com, x, y) = (line[0], line[1] as usize, line[2] as usize);
        if com == 0 { // unite
            uf.unionset(x, y);
        } else { // same
            println!("{}",uf.same(x,y) as i32);
        }
    }
}
