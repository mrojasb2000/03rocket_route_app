#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use rocket::{Build, Rocket};
use std::collections::HashMap;

#[derive(Debug)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    active: bool,
}

lazy_static! {
    static ref USERS: HashMap<&'static str, User> = {
        let mut map = HashMap::new();
        map.insert(
            "3e3dd4ae-3c37-40c6-aa64-7061f284ce28",
            User {
                uuid: String::from("3e3dd4ae-3c37-40c6-aa64-7061f284ce28"),
                name: String::from("John Doe"),
                age: 18,
                active: true,
            },
        );
        map.insert(
            "6fabdd30-f6e8-4bb0-af4d-b7bd321d4c56",
            User {
                uuid: "6fabdd30-f6e8-4bb0-af4d-b7bd321d4c56".to_string(),
                name: "Bob Doe".to_string(),
                age: 25,
                active: false,
            },
        );
        map.insert(
            "cfd3e221-77fa-4568-be5a-69df621ffce4",
            User {
                uuid: "cfd3e221-77fa-4568-be5a-69df621ffce4".to_string(),
                name: "Charlie Doe".to_string(),
                age: 35,
                active: true,
            },
        );
        map
    };
}

#[get("/user/<uuid>", rank = 1, format = "text/plain")]
fn user(uuid: &str) -> String {
    let user = USERS.get(uuid);
    match user {
        Some(u) => format!("Found user: {:?}", u),
        None => String::from("User not found"),
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![user])
}
