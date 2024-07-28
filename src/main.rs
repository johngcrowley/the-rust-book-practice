use core::panic;
use std::collections::HashMap;
use std::io;

// Challenge 1: ->  Median and Mode
//     Given a list of integers, use a vector and return the median
//     (when sorted, the value in the middle position)
//     and mode (the value that occurs most often; a hash map will be
//     helpful here) of the list.

fn median(int_list: &mut [i32]) -> Option<i32> {
    let midway: usize = ((int_list.len()) as f64 / 2.0).ceil() as usize;

    // Confused on 'enumerate' being a 'self' method: it seems that would consume
    // the reference that 'int_list.iter()' returns. Is that happening and we're
    // getting int_list.len() * 2 copies of each integer?
    for (i, v) in int_list.iter().enumerate() {
        if i == midway {
            return Some(*v as i32);
        }
    }
    None
}

fn mode(int_list: &mut [i32]) -> Option<i32> {
    let mut mapper = HashMap::new();
    let mut biggest: Option<i32> = None;
    let mut freq: i32 = 1;
    for val in int_list.iter() {
        mapper
            // returns an <Entry<K,V>> which is an Option<>
            .entry(val)
            // takes a closure to update an Entry
            .and_modify(|count| *count += 1)
            // adds the 'val' if missing, setting it to '1' in this case.
            .or_insert(1);
        if mapper[val] > freq {
            freq = mapper[val];
            biggest = Some(*val);
        }
    }
    biggest
}

fn median_and_mode(int_list: &mut [i32]) -> (i32, i32) {
    int_list.sort();

    let median = match median(int_list) {
        Some(m) => m,
        None => {
            panic!("No median found in int_list! {:?}", int_list)
        }
    };
    //println!("[median_and_mode]{:p}", &median);
    let mode = match mode(int_list) {
        Some(m) => m,
        None => {
            panic!("No mode found in int_list! {:?}", int_list)
        }
    };

    (median, mode)
}

// Challenge 2: -> Convert strings to pig latin.
//     The first consonant of each word is moved to the end of the word and “ay” is added,
//     so “first” becomes “irst-fay.”
//     Words that start with a vowel have “hay” added to the end instead
//     (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
fn is_a_vowel(l: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    vowels.iter().any(|&v| v == *l)
}
fn pig_latin(str_list: &mut [String]) {
    // each 'word' is a mutable reference to heap String of the vector.
    // we can't move out of those references, but we can change their values.
    let mut tail_end: &str = "";
    for word in str_list.iter_mut() {
        let mut new_word = String::with_capacity(word.len());
        for (i, c) in word.chars().enumerate() {
            if i == 0 {
                if is_a_vowel(&c) {
                    tail_end = "-hay";
                    // Can't use '+=' to concatenate a 'char' to a String.
                    new_word.push(c);
                } else {
                    tail_end = "-ay";
                }
            } else {
                new_word.push(c);
            }
        }
        new_word += tail_end;
        // This is legal. We're moving the heap value 'new_word'
        // to overwrite the value of 'word'
        // We're _not_ setting the value of a reference ('word') to
        // a new stack reference, which results in a double-free at the end of 'main()'.
        *word = new_word;
    }
}

// Challenge 3: -> Make a user interface for a company's employees.
//     Using a hash map and vectors, create a text interface to allow a user to add employee
//     names to a department in a company.
//     For example, “Add Sally to Engineering” or “Add Amir to Sales.”
//     Then let the user retrieve a list of all people in a department or
//     all people in the company by department, sorted alphabetically.

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        let mut company = HashMap::new();
        for department in ["sales", "marketing", "accounting"] {
            company.insert(department.to_string(), vec![]);
        }
        Company {
            departments: company,
        }
    }

    fn add_department(&mut self, dept: &str) {
        self.departments.insert(dept.to_string(), vec![]);
    }

    fn get_department(&self, string: &str) {
        println!("Department: {string}");
        println!("Employees in {string}: {:?}", self.departments[string]);
    }

    fn add_to_department(&mut self, emp: &str, dept: &str) {
        self.departments
            .entry(dept.to_string())
            .or_insert(vec![])
            .push(emp.to_string());
    }

    fn get_company(&self) {
        for (k, v) in self.departments.iter() {
            println!("--- Department: {k} ---");
            for employee in v.iter() {
                println!("{employee}");
            }
        }
    }
}

fn company_interface() {
    // Instantiate a 'Company'
    // All Strings created in new() remain, their owner has moved to
    // the stack reference of the path, 'at'.
    let mut at = Company::new();

    let usage: &str = "Usage: get <department> | get company | add <employee> to <department>";

    loop {
        println!(
            "Which action would you like to take?
        - get <department>
        - get <company>
        - add <person> to <department>
        "
        );

        let mut input_buffer = String::new();
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Must provide valid input.");

        let mut command_book = HashMap::new();
        for (i, v) in input_buffer
            .split_ascii_whitespace()
            .into_iter()
            .enumerate()
        {
            command_book.insert(i, v.to_lowercase());
        }

        let commands = command_book.len();

        if commands == 2 {
            match command_book.get(&0).map(|x| x.as_str()) {
                Some("add") => {
                    at.add_department(command_book.get(&1).expect(&usage));
                }
                Some("get") => match command_book.get(&1).map(|x| x.as_str()) {
                    Some("company") => {
                        at.get_company();
                    }
                    _ => {
                        at.get_department(command_book.get(&1).expect(&usage));
                    }
                },
                _ => {
                    panic!("{}", usage);
                }
            }
        } else if commands == 4 {
            match command_book.get(&0).map(|x| x.as_str()) {
                Some("add") => match command_book.get(&2).map(|x| x.as_str()) {
                    Some("to") => {
                        at.add_to_department(
                            command_book.get(&1).expect(&usage),
                            command_book.get(&3).expect(&usage),
                        );
                    }
                    _ => {
                        panic!("{}", usage);
                    }
                },
                _ => {
                    panic!("{}", usage);
                }
            }
        } else {
            panic!("{}", usage);
        }
    }
}

fn main() {
    // Median and Mode ///////////////////////////////////////////////////
    let mut int_list = vec![45, 69, 420, 3, 21, 47, 21, 10, 51];
    let (median, mode) = median_and_mode(&mut int_list);
    println!("Median: {median}, Mode: {mode} of {:?}", &int_list);

    // Pig Latin ////////////////////////////////////////////////////////
    let mut str_list = vec![
        String::from("cello"),
        String::from("bello"),
        String::from("hello"),
        String::from("apple"),
    ];
    println!("OG list: {:?}", str_list);
    pig_latin(&mut str_list);
    println!("Pig list: {:?}", str_list);

    // Departments //////////////////////////////////////////////////////
    company_interface();
}
