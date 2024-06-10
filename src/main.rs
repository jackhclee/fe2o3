use domain::*;

fn main() {
    println!("Hello, world!");
    let book = Book {
        title: String::from("AAA"),
        price: 100
    };
    println!("{:?} discounted_price ${}", book, book.discounted_price(0.9));
}

pub mod domain {

    #[derive(Debug)]
    pub struct Book {
        pub title: String,
        pub price: i32
    }

    impl Book {
        pub fn discounted_price(&self, discount_rate: f32) -> f32 {
            self.price as f32 * discount_rate
        }
    }
}
