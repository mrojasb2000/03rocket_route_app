#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use rocket::http::ContentType;
use rocket::response::{self, Responder, Response};
use rocket::{request::FromParam, Request};
use rocket::{Build, Rocket};
use std::collections::HashMap;
use std::io::Cursor;
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

fn default_response<'r>() -> response::Response<'r> {
    Response::build()
        .header(ContentType::Plain)
        .raw_header("X-CUSTOM-ID", "CUSTOM")
        .finalize()
}

impl<'r> Responder<'r, 'r> for &'r User {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let base_response = default_response();
        let user = format!("Found user: {:?}", self);
        Response::build()
            .sized_body(user.len(), Cursor::new(user))
            .raw_header("X-USER-ID", self.uuid.to_string())
            .merge(base_response)
            .ok()
    }
}

// newtype definition
struct NewUser<'a>(Vec<&'a User>);

impl<'r> Responder<'r, 'r> for NewUser<'r> {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'r> {
        let base_response = default_response();
        let user = self
            .0
            .iter()
            .map(|u| format!("{:?}", u))
            .collect::<Vec<String>>()
            .join(",");
        Response::build()
            .sized_body(user.len(), Cursor::new(user))
            .raw_header("X-CUSTOM-ID", "USERS")
            .join(base_response)
            .ok()
    }
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
fn user(uuid: &str) -> Option<&User> {
    let user = USERS.get(uuid);
    match user {
        Some(u) => Some(u),
        None => None,
    }
}

// #[route(GET, uri = "/users/<grade>?<filters..>")]
// fn users(grade: u8, filters: Filters) {}

#[get("/users/<name_grade>?<filters..>")]
fn users(name_grade: NameGrade, filters: Option<Filters>) -> Option<NewUser> {
    let users: Vec<&User> = USERS
        .values()
        .filter(|user| user.name.contains(&name_grade.name) && user.grade == name_grade.grade)
        .filter(|user| {
            if let Some(fts) = &filters {
                user.age == fts.age && user.active == fts.active
            } else {
                true
            }
        })
        .collect();
    if users.len() > 0 {
        Some(NewUser(users))
    } else {
        None
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![user, users])
}
