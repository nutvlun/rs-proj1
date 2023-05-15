#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;



fn mul2(num: &u8)->u8 {
    return num*2;
}

#[get("/hello/<name>/<age>/<cool>")]
fn world(name : String, age: u8, cool: bool) -> String  {
   
    if cool {
       let num = mul2(&age);
       format!("You're a cool {} year old, {}!", num, name)

    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
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
