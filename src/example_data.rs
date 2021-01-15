use crate::Contact;
use druid::ImageBuf;

pub fn get_data() -> Vec<Contact> {
    vec![
        Contact {
            name: "Billy Bob".to_string(),
            email: "Billybob@gmail.com".to_string(),
            image: ImageBuf::empty(),
            favorite_food: "Curry".to_string(),
            age: 39,
        },
        Contact {
            name: "Waka waka".to_string(),
            email: "wakaka@gmail.com".to_string(),
            image: ImageBuf::empty(),
            favorite_food: "Fried Rice".to_string(),
            age: 65,
        },
        Contact {
            name: "Chance Rapper".to_string(),
            email: "chancerapper@gmail.com".to_string(),
            image: ImageBuf::empty(),
            favorite_food: "Brussel Sprouts".to_string(),
            age: 22,
        },
        Contact {
            name: "Vincente Fernandez".to_string(),
            email: "VFernandez@gmail.com".to_string(),
            image: ImageBuf::empty(),
            favorite_food: "Rice and Beans".to_string(),
            age: 51,
        },
    ]
}
