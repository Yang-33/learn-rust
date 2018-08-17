// 4-30
// associated-types

fn mode001() {
    // a-----
    // これは一見良さそうだけどそんなに良くない
    trait Graph<N, E> {
        fn has_edge(&self, &N, &N) -> bool;
        fn edges(&self, &N) -> Vec<E>;
        // etc
    }


    // えーってなる
    fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 { 0  }
}


fn mode002(){
    trait Graph {
        type N;
        type E;

        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
        // etc
    }

    // ちょっとスッキリする
    fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> u32 {  0 }
}


// こんなかんじになる
// trait type が結構良さそう
fn mode003(){

    trait Graph {
        type N;
        type E;

        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
        // etc
    }

    struct Node;

    struct Edge;

    struct MyGraph;

    impl Graph for MyGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
            true
        }

        fn edges(&self, n: &Node) -> Vec<Edge> {
            Vec::new()
        }
    }
}

fn main(){
    mode003();
}