use domain::*;

fn print_book(discounted: &impl Discounted) -> () {
    println!("This is print_book {}", discounted.less(0.9))
}

fn main() {
    println!("Hello, world!");
    let mut book = Book {
        title: String::from("AAA"),
        price: 100,
    };

    print_book(&book);
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
}

pub mod domain {

    pub trait Discounted {
        fn discounted_price(&self, discount_rate: f32) -> f32;

        fn less(&self, discount_rate: f32) -> f32;

        fn update_price(&mut self, discount_rate: f32) -> ();
    }

    #[derive(Debug)]
    pub struct Book {
        pub title: String,
        pub price: i32,
    }

    impl Discounted for Book {
        fn discounted_price(&self, discount_rate: f32) -> f32 {
            self.price as f32 * discount_rate
        }

        fn less(&self, discount_rate: f32) -> f32 {
            self.price as f32 - self.discounted_price(discount_rate)
        }

        fn update_price(&mut self, discount_rate: f32) -> () {
            self.price = self.discounted_price(discount_rate) as i32;
        }
    }
}
