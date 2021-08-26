use std::collections::HashMap;

fn main() {
    // Exercise 1: Mean, Median, Mode
    exercise1();

    // Exercise 2: Pig Latin
    exercise2();

    // Exercise 3: Employees List
    exercise3()
}

////////////////////////////////////////////////////////////
// Exercise 1: Mean, Median, Mode
fn exercise1() {
    let v = vec![-5, 1, 2, 2, 5, 3, 5, 5];

    println!("v: {:?}", v);
    println!("mean: {}", mean(&v));

    println!("median: {}", median(&v));

    // mode-1: for loop
    println!("mode-2 (for loop): {:?}", mode_1(&v));

    // mode-2: w/ Iterator::max_by_key() method
    println!(
        "mode-2 (w/ Iterator::max_by_key() method): {:?}",
        mode_2(&v)
    );

    // mode-3: w/ Iterator::reduce() method
    println!("mode-3 (w/ IIterator::reduce() method): {:?}", mode_3(&v));
}

fn mean(v: &Vec<i32>) -> f32 {
    let sum: i32 = v.iter().sum();
    sum as f32 / v.len() as f32
}

fn median(v: &Vec<i32>) -> f32 {
    // copy to keep the original!
    //let mut v: Vec<i32> = v.iter().copied().collect();
    let mut v = v.clone();

    // sort
    v.sort();
    println!("sorted v: {:?}", v);

    // median
    let m_index = v.len() / 2;
    if v.len() % 2 == 0 {
        ((v[m_index - 1] + v[m_index]) as f32) / 2.0
    } else {
        v[m_index] as f32
    }
}

// mode-1: for loop
fn mode_1(v: &Vec<i32>) -> Option<i32> {
    let hist = histgram(v);
    println!("hist: {:?}", hist);

    // find the peak
    let mut peak = None;
    let mut max = 0;
    // WHY dereference?
    for (&k, &v) in hist.iter() {
        if v > max {
            peak = Some(k);
            max = v;
        }
    }

    peak
}

// mode-2: w/ Iterator::max_by_key() method
fn mode_2(v: &Vec<i32>) -> Option<i32> {
    let hist = histgram(v);
    println!("hist: {:?}", hist);

    let peak = hist.iter().max_by_key(|(_, v)| *v);
    match peak {
        None => None,
        Some((k, _)) => Some(*k),
    }
}

// mode-3: w/ Iterator::reduce() method
fn mode_3(v: &Vec<i32>) -> Option<i32> {
    let hist = histgram(v);
    println!("hist: {:?}", hist);

    let peak = hist.iter().reduce(|a, b| if a.1 > b.1 { a } else { b });
    match peak {
        None => None,
        Some((k, _)) => Some(*k),
    }
}

fn histgram(v: &Vec<i32>) -> HashMap<i32, u32> {
    let mut hist = HashMap::new();

    // make a histgram
    for e in v {
        let count = hist.entry(*e).or_insert(0);
        *count += 1;
    }

    hist
}

////////////////////////////////////////////////////////////
// Exercise 2: Pig Latin
fn exercise2() {
    let word = String::from("apple");
    println!("{} => {}", word, pig_latin(&word));

    let word = String::from("first");
    println!("{} => {}", word, pig_latin(&word));
}

fn pig_latin(word: &str) -> String {
    let vowels: &[char] = &['a', 'i', 'u', 'e', 'o'];
    if word.starts_with(vowels) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", &word[1..], &word[0..1])
    }
}

////////////////////////////////////////////////////////////
// Exercise 3: Employees List
fn exercise3() {
    let mut list = EmployeesList::new();

    // add departments
    list.add_department("Marketing");
    list.add_department("Operations");
    list.add_department("Finance");
    list.add_department("Sales");
    list.add_department("Purchase");
    list.show_departments();

    // add employees
    list.add_employee("Marketing", "weil");
    list.add_employee("Marketing", "nico");
    list.show_members_of_department("Marketing");
    list.add_employee("Operations", "tally");
    list.add_employee("Operations", "serre");
    list.show_members_of_department("Operations");
    list.add_employee("Finance", "gauss");
    list.show_members_of_department("Finance");
    list.add_employee("Sales", "riemann");
    list.add_employee("Sales", "euler");
    list.add_employee("Sales", "galois");
    list.add_employee("Sales", "abel");
    list.show_members_of_department("Sales");
    list.add_employee("Purchase", "fermat");
    list.add_employee("Purchase", "euler");
    list.add_employee("Purchase", "poincare");
    list.show_members_of_department("Purchase");

    // show all members
    list.show_all();
}

struct EmployeesList {
    // map: a Department -> a List of members
    map: HashMap<String, Vec<String>>,
}

impl EmployeesList {
    fn new() -> EmployeesList {
        let list = EmployeesList {
            map: HashMap::new(),
        };
        list
    }

    fn add_department(&mut self, d: &str) {
        self.map.entry(d.to_string()).or_insert(Vec::new());
    }

    fn show_departments(&self) {
        let deps: Vec<&String> = self.map.keys().collect();
        println!("Departments: {:?}", deps);
    }

    fn add_employee(&mut self, d: &str, e: &str) {
        match self.map.get_mut(d) {
            Some(vec) => {
                vec.push(e.to_string());
                // Sort in every insertion!
                vec.sort();
            }
            None => {
                println!("{} Department doesn't exist!", d);
            }
        }
    }

    fn show_members_of_department(&self, d: &str) {
        match self.map.get(d) {
            Some(vec) => {
                println!("Members of {} Department: {:?}", d, vec);
            }
            None => {
                println!("{} Department doesn't exist!", d);
            }
        }
    }

    // all people in the company by department, sorted alphabetically
    fn show_all(&self) {
        for (dep, members) in self.map.iter() {
            println!("{} Department:", dep);
            for employee in members {
                println!("{}", employee);
            }
            println!("");
        }
    }
}
