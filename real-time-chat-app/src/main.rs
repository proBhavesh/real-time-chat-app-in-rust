#[macro_use] extern crate rocket;

#[get("/world")]
fn world()->&'static str{
    "Hello, world!"
}

#[launch]
fn rocket()->_{
    rocket::build()
        .mount("/hello", routes![world])
}

#[post("/message", data="<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>){
    let _res=queue.send(form.to_inner())
}