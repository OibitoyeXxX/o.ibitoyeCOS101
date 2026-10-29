fn main() {
    let v = vec![10, 20, 30];
    // vector v owns the object in heap

    let v2 = v; // moves ownership to v2

    display(v2);
    // v2 is moved to display and v2 is invalidated

    // v2 is no longer usable here, so print a message instead
    println!("In main: v2 was moved into display");
}

fn display(v: Vec<i32>) {
    println!("inside display {:?}", v);
}