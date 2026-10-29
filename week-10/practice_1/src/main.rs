fn main() {
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    let v2 = v;
    // ownership moved from v to v2

    // print v2, the new owner (v is no longer usable)
    println!("{:?}", v2);
}