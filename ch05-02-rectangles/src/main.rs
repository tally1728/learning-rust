fn main() {
    // w/o Tuple or Struct
    example1_wo_struct();

    // w/ Tuple
    example2_w_tuple();

    // w/ Struct
    example3_w_struct();
}

// w/o Tuple or Struct
fn example1_wo_struct() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// w/ Tuple
fn example2_w_tuple() {
    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(rect)
    );
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// w/ Struct
fn example3_w_struct() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    //println!("rect1 is {}", rect);

    // if w/o #[derive(Debug)]
    // error[E0277]: `Rectangle` doesn't implement `Debug`
    println!("rect1 is {:?}", rect);
    println!("rect1 is {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_3(&rect)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
