fn main() {
    // Listing 8-11, 8-12, 8-13
    // generate strings
    example_generate_strings();

    // Listing 8-14: Hello in many languages
    example_strings_in_many_languages();

    // Listing 8-15 & 8-17: Appending to a String with push_str and push
    example_push();

    // Listing 8-18: Concatenation with the + Operator or the format! Macro
    example_concate();

    // Internal Representation
    example_slice();

    // Iterating Over Strings
    example_iterating_over_strings();

    extra();
}

// Listing 8-11, 8-12, 8-13
// generate strings
fn example_generate_strings() {
    // generate emptry String
    let s = String::new();
    println!("{}", s);

    // generate String w/ to_string method
    let data = "initial contents"; // &str
    let s = data.to_string(); // String
    println!("{}", s);

    // generate String w/ from function
    let data = "initial contents"; // &str
    let s = String::from(data); // String
    println!("{}", s);
}

// Listing 8-14: Hello in many languages
fn example_strings_in_many_languages() {
    let mut hellos = Vec::new();

    hellos.push(String::from("السلام عليكم"));
    hellos.push(String::from("Dobrý den"));
    hellos.push(String::from("Hello"));
    hellos.push(String::from("שָׁלוֹם"));
    hellos.push(String::from("नमस्ते"));
    hellos.push(String::from("こんにちは"));
    hellos.push(String::from("안녕하세요"));
    hellos.push(String::from("你好"));
    hellos.push(String::from("Olá"));
    hellos.push(String::from("Здравствуйте"));
    hellos.push(String::from("Hola"));

    for hello in &hellos {
        println!("{}", hello);
    }
}

// Listing 8-15 & 8-17: Appending to a String with push_str and push
fn example_push() {
    // push_str method
    let mut s = String::from("foo");
    println!("{}", s);

    s.push_str("bar");
    println!("{}", s);

    // push_str method
    let mut s = String::from("lo");
    println!("{}", s);

    s.push('l');
    println!("{}", s);
}

// Listing 8-18: Concatenation with the + Operator or the format! Macro
fn example_concate() {
    // + Operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // fn add(self, s: &str) -> String
    // ownership moved from s1 to s3
    let s3 = s1 + &s2;
    println!("s3: {}", s3);

    // concate multiple strings w/ + operator
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    // concate multiple strings w/ + format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s1: {}", s1); // s1 still owns!!
    println!("s: {}", s);
}

// Internal Representation
fn example_slice() {
    // 1 byte letter
    let str = String::from("Hola");
    println!("str: {} (length: {})", str, str.len());
    let byte: &str = &str[0..1];
    println!("s: {}", byte);

    // Cyrillic: 2 bytes letter
    let str = String::from("Здравствуйте");
    println!("str: {} (length: {})", str, str.len());
    // runtime error: byte index 1 is not a char boundary
    //let byte: &str = &str[0..1];
    let bytes: &str = &str[0..2];
    println!("s: {}", bytes);

    // Devanagari: 3 bytes letter
    let str = String::from("नमस्ते");
    println!("str: {} (length: {})", str, str.len());
    let bytes: &str = &str[0..3];
    println!("&str[0..3]: {}", bytes);
    let bytes: &str = &str[9..12];
    println!("&str[9..12]: {}", bytes);
    let bytes: &str = &str[9..];
    println!("&str[9..]: {}", bytes);

    // Kanji: 3 bytes letter
    let str = String::from("夕食");
    println!("str: {} (length: {})", str, str.len());
    let bytes: &str = &str[0..3];
    println!("&str[0..3]: {}", bytes);
    let bytes: &str = &str[3..6];
    println!("&str[3..6]: {}", bytes);
}

// Iterating Over Strings
fn example_iterating_over_strings() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn extra() {
    let mut s = String::new();

    // "ते" is Not Unicode letter, But Grapheme cluster
    // s.push('ते');
    s.push_str("ते"); // OK!
    println!("{}", s);

    // these are parts of Grapheme cluster "ते"
    s = String::new();
    s.push('्');
    s.push('त');
    s.push('े');
    println!("{}", s);

    s = String::new();
    s.push('食');
    println!("{}", s);
}
