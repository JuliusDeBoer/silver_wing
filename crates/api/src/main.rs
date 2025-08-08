#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello from the API!"
}

#[get("/kitty")]
fn kitty() -> &'static str {
    "                                     ,
              ,-.       _,---._ __  / \\
             /  )    .-'       `./ /   \\
            (  (   ,'            `/    /|
             \\  `-\"             \\'\\   / |
              `.              ,  \\ \\ /  |
               /`.          ,'-`----Y   |
              (            ;        |   '
              |  ,-.    ,-'         |  /
              |  | (   |        hjw | /
              )  |  \\  `.___________|/
              `--'   `--'"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, kitty])
}
