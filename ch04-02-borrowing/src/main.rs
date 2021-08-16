fn main() {
    // Function & Ownership
    function_and_ownership_3();

    // Immutable Ref
    immutable_refs();
    immutable_refs_2();

    // Mutable Ref
    mutable_ref_1();
    mutable_ref_2();
    mutable_ref_3();

    // Dangling References
    //reference_to_nothing();
}

////////////////////////////////////////////////////////////
// Immutable Ref
//
fn function_and_ownership_3() {
    let s = String::from("hello");

    // gives reference... Efficient way!
    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();

    length
}

// 2 Immutable Refs
fn immutable_refs() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("s(Owner): {}", s);
    println!("r1(Immutable Reference): {}", r1);
    println!("r2(Immutable Reference): {}", r2);
}

// 2 Immutable Refs
fn immutable_refs_2() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = r1; // Copy of Reference

    println!("s(Owner): {}", s);
    println!("r1(Immutable Reference): {}", r1);
    println!("r2(Immutable Reference): {}", r2);
}

////////////////////////////////////////////////////////////
// Mutable Ref
//
fn mutable_ref_1() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// 2 Mut Refs: NG!!
fn mutable_ref_2() {
    let mut s = String::from("hello");

    // 1st Mut Ref: OK
    let r1 = &mut s;
    // 2nd Mut Ref: NG
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    //let r2 = &mut s;

    r1.push_str(", world!");

    //println!("s(Owner): {}", s);
    println!("r1(Mutable Reference): {}", r1);
}

// Immutable Ref & Mut Ref: NG!!
fn mutable_ref_3() {
    let s = String::from("hello");

    // 1st: Immutable Ref
    let r1 = &s;
    // 2nd: Mutable Ref
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //let r2 = &mut s;

    println!("s(Owner): {}", s);
    println!("r1(Immutable Reference): {}", r1);
}

// Dangling References
// fn reference_to_nothing() {
//     let r = dangling();
//
//     println!("r(): {}", r);
// }

// error[E0106]: missing lifetime specifier
// fn dangling() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
