use fe2o3::domain::*;
use log::{info};
use serde::Serialize;

fn print_book(discounted: &impl Discounted, ctr: &mut i32) -> () {
    *ctr += 1;
    println!("This is print_book {}", discounted.less(0.9));
    println!("This is print_book {}", discounted.less(0.9))
}


fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("HELLO WORLD");

    let mut prog_ctr: i32 = 0;
    println!("Hello, world!");
    let mut book = Book {
        title: String::from("AAA"),
        price: 100,
    };

    let p1 = Person { person_id: 678 , nick_name: None };

    println!("{:?}", p1);
    println!("{}", serde_json::to_string(&p1).unwrap());

    println!("{}", serde_yaml::to_string(&p1).unwrap());


    println!("{}", Option::is_none(&p1.nick_name));



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
