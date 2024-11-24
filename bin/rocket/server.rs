// https://github.com/rwf2/Rocket

use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[macro_use]
extern crate rocket;

#[get("/hello")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    message: String,
}

#[post("/json", data = "<message>")]
fn json(message: Json<Message>) -> String {
    format!("Message: {}", message.message)
}

#[derive(FromForm)]
struct Query {
    name: String,
    age: u8,
}

#[get("/query?<params..>")]
fn query(params: Query) -> String {
    format!("Name: {}, Age: {}", params.name, params.age)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // curl -XGET "127.0.0.1:8000/hello"
        .mount("/", routes![hello_world])
        // curl -XPOST -d '{"message": "hello"}' "127.0.0.1:8000/json"
        .mount("/", routes![json])
        // curl -XGET "127.0.0.1:8000/query?name=John&age=20"
        .mount("/", routes![query])
}
