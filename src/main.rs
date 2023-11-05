#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_dyn_templates;

use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/")]
fn tensorflow_docs() -> Template {
    Template::render("tensorflow", context! {})
}

#[get("/")]
fn tensorflowlite_docs() -> Template {
    Template::render("tensorflowlite", context! {})
}

#[get("/")]
fn opencv_install() -> Template {
    Template::render("opencv_install", context! {})
}

#[get("/")]
fn tflite_install() -> Template {
    Template::render("tflite_install", context! {})
}

#[get("/")]
fn libedgetpu_install() -> Template {
    Template::render("libedgetpu_install", context! {})
}

#[get("/")]
fn cmake_install() -> Template {
    Template::render("cmake_install", context! {})
}

#[get("/")]
fn ort_install() -> Template {
    Template::render("ort_install", context! {})
}

#[get("/")]
fn render_css() -> Template {
    Template::render("style", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/tensorflow", routes![tensorflow_docs])
        .mount("/tensorflowlite", routes![tensorflowlite_docs])
        .mount("/opencv_install", routes![opencv_install])
        .mount("/tflite_install", routes![tflite_install])
        .mount("/libedgetpu_install", routes![libedgetpu_install])
        .mount("/cmake_install", routes![cmake_install])
        .mount("/onnxruntime_install", routes![ort_install])
        .mount("/style.css", routes![render_css])
        .attach(Template::fairing())
}
