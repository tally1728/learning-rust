fn main() {
    panic2();
}

fn panic1() {
    panic!("crash and burn");
}

fn panic2() {
    let v = vec![0, 1, 2];
    v[99];
}
