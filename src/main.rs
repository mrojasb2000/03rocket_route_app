#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use rocket::{request::FromParam, Build, Rocket};
use std::collections::HashMap;
use std::vec::Vec;

#[derive(FromForm)]
struct Filters {
    age: u8,
    active: bool,
}

struct NameGrade<'r> {
    name: &'r str,
    grade: u8,
}

impl<'r> FromParam<'r> for NameGrade<'r> {
    type Error = &'static str;
    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        const ERROR_MESSAGE: Result<NameGrade, &'static str> = Err("Error parsing user parameter");
        let name_grade_vec: Vec<&'r str> = param.split('_').collect();
        match name_grade_vec.len() {
            2 => match name_grade_vec[1].parse::<u8>() {
                Ok(n) => Ok(Self {
                    name: name_grade_vec[0],
                    grade: n,
                }),
                Err(_) => ERROR_MESSAGE,
            },
            _ => ERROR_MESSAGE,
        }
    }
}

#[derive(Debug)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    grade: u8,
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
                grade: 1,
                active: true,
            },
        );
        map.insert(
            "6fabdd30-f6e8-4bb0-af4d-b7bd321d4c56",
            User {
                uuid: "6fabdd30-f6e8-4bb0-af4d-b7bd321d4c56".to_string(),
                name: "Bob Doe".to_string(),
                age: 25,
                grade: 2,
                active: false,
            },
        );
        map.insert(
            "cfd3e221-77fa-4568-be5a-69df621ffce4",
            User {
                uuid: "cfd3e221-77fa-4568-be5a-69df621ffce4".to_string(),
                name: "Charlie Doe".to_string(),
                grade: 3,
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

// #[route(GET, uri = "/users/<grade>?<filters..>")]
// fn users(grade: u8, filters: Filters) {}

#[get("/users/<name_grade>?<filters..>")]
fn users(name_grade: NameGrade, filters: Filters) -> String {
    let users: Vec<&User> = USERS
        .values()
        .filter(|user| user.name.contains(&name_grade.name) && user.grade == name_grade.grade)
        .filter(|user| user.age == filters.age && user.active == filters.active)
        .collect();
    if users.len() > 0 {
        users
            .iter()
            .map(|u| u.name.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    } else {
        String::from("No users found")
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![user, users])
}
