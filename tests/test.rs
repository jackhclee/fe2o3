use fe2o3::domain::{Book, Discounted, Person, serialize_person};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discounted_price() {
        let book = Book {
            title: String::from("My Book"),
            price: 100,
        };

        assert_eq!(book.discounted_price(0.9), 90.0);
        assert_eq!(book.discounted_price(0.5), 50.0);
    }

    #[test]
    fn test_serialize_person() {
        let person = Person {
            person_id: 123,
            nick_name: Some("tester".to_string()),
        };
        let json = serialize_person(&person);
        assert_eq!(json, r#"{"personId":123,"nickName":"tester"}"#);
    }
}