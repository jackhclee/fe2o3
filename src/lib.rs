use serde::Serialize;

pub mod domain {

    use serde::Serialize;

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

    #[derive(Debug, Serialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Person {
        pub person_id: i64,

        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nick_name: Option<String>
    }

    pub fn serialize_person(person: &Person) -> String {
        serde_json::to_string(&person).unwrap()
    }
}
