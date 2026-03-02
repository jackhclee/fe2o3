use fe2o3::domain::*;
use im::{vector, Vector};
use log::info;
use serde::Serialize;
use std::thread;

fn test_box() {
    let s = String::from("BOXED");
    let b = Box::new(s);

    println!("Hello, world! {}", b);
}

struct Car {
    name: String,
}

impl Car {
    fn run(&self) {
        println!("Run {}", self.name);
    }
}
fn print_book(discounted: &impl Discounted, ctr: &mut i32) -> () {
    *ctr += 1;
    println!("This is print_book {}", discounted.less(0.9));
    println!("This is print_book {}", discounted.less(0.9));
    panic!("yo");
}

#[derive(Serialize)]
struct Log {
    level: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dest: Option<String>,
}

#[derive(Debug)]
struct Slots(i32, i32);

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    println!("Enter");
    let mut n = String::new();
    let r = std::io::stdin().read_line(&mut n);
    println!("{:?}", n);
    let i: i32 = 1;
    println!("Division result: {:?}", i.checked_div(n.trim().parse::<i32>().unwrap()));

    let t = (1, 2, 3);

    println!("i32 default: {}", i32::default());
    println!("tuple: {}", t.0);
    test_box();
    info!("HELLO WORLD");

    let arr = [1, 2, 3];

    println!("{:?}", arr);
    println!("{:?}", arr.len());
    println!("{:?}", &arr[1..3]);

    println!("{:?}", &arr[0..3]);

    let s = Slots(1, 2);

    println!("Slots: {:?}", s);

    let c = Car {
        name: String::from("Toyota"),
    };

    c.run();

    let mut sr: String = String::new();

    println!("sr {}", sr);

    let mut v = Vector::new();
    // ![1, 2, 3];
    v.push_back(2);
    // v.push_back(3);

    println!("v is {:?}", v);

    println!("vector v all even: {}", v.iter().all(|x| x % 2 == 0));

    println!(
        "vector v all even: {}",
        v.iter().map(|y| y + 1).all(|x| x % 2 == 0)
    );

    let mut prog_ctr: i32 = 0;
    println!("Hello, world!");
    let mut book = Book {
        title: String::from("AAA"),
        price: 100,
    };

    let p1 = Person {
        person_id: 678,
        nick_name: None,
    };

    println!("{:?}", p1);
    println!("{}", serde_json::to_string(&p1).unwrap());

    println!("{}", serde_yaml::to_string(&p1).unwrap());

    let l1 = Log {
        level: 1,
        dest: None,
    };

    println!("Log:");
    println!("{}", serde_json::to_string(&l1).unwrap());

    let mut v = vec![1, 2, 3];

    println!("before: {:?}", v);

    let t = thread::spawn(move || {
        v[2] = 5;
        v.clone()
    });

    let nv = t.join().unwrap();

    println!("after: {:?}", nv);

    print_book(&book, &mut prog_ctr);
    print_book(&book, &mut prog_ctr);
    println!(
        "{:?} discounted_price ${} (less ${})",
        book,
        book.discounted_price(0.9),
        book.less(0.9)
    );
    book.update_price(0.9);
    println!(
        "{:#?} discounted_price ${} (less ${})",
        book,
        book.discounted_price(0.9),
        book.less(0.9)
    );

    println!("prog_ctr {}", prog_ctr)
}
