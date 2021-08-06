fn main() {
    // Exercise-1
    // Fahrenheit to Celsius
    let f_temp: f64 = 70.0;
    let c_temp = f2c(f_temp);
    println!("{} deg in F = {} deg in C", f_temp, c_temp);

    // Celsius to Fahrenheit
    let c_temp: f64 = 36.0;
    let f_temp = c2f(c_temp);
    println!("{} deg in C = {} deg in F", c_temp, f_temp);

    // Exercise-2: Fibonacci
    let index = 6;
    println!("{}th of Fibonacci Seqence: {}", index, fibonacci(index));

    // Exercise-3: The Twelve Days of Christmas
    the_twelve_days_of_christmas();
}

// Fahrenheit to Celsius
fn f2c(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

// Celsius to Fahrenheit
fn c2f(t: f64) -> f64 {
    t * 9.0 / 5.0 + 32.0
}

// n-th Fibonacci number
fn fibonacci(n: u32) -> u32 {
    // nothing to do for 0, 1
    if n < 2 {
        return n;
    };

    // last 2 elements of Fibonacci sequence
    let mut seq = [0, 1];

    for _ in 0..n - 1 {
        seq = [seq[1], seq[0] + seq[1]];
    }

    seq[1]
}

// The Twelve Days of Christmas
fn the_twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sexth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let parts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12 {
        println!("[Verse {}]", i + 1);
        println!(
            "On the {} day of Christmas, my true love sent to me",
            days[i]
        );

        for j in (0..=i).rev() {
            println!("{}", parts[j]);
        }

        println!("");
    }
}
