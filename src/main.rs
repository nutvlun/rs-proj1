#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


#[get("/hello")]
fn world() -> &'static str {
    "Hello, world!"
}
#[get("/test")]
fn test() -> &'static str {
    "Hello, Test!"
}
#[get("/test-cargo")]
fn test_cargo() -> &'static str {
    "Cargo, Test!"
}
#[get("/test-echo")]
fn test_echo() -> &'static str{
    "Cargo, Echo!"
}
#[get("/")]
fn index() -> &'static str{
    "Cargo, Index!"
}
fn main() {
    rocket::ignite()
    .mount("/", routes![world,test,test_cargo])
    .mount("/cc", routes![index,test_echo])
    .launch();
}
