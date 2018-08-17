// 4-31
// unsized-types


fn mode001() {
    struct Foo<T: ?Sized> {
        f: T,
    }
}

fn main() {
    mode001();
}