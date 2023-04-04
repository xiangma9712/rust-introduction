mod gcd_lib;

use crate::gcd_lib::handlers;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding to address")
        .run()
        .expect("error running server");
}

fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="POST">
                <input type="text" name="m" />
                <input type="text" name="n" />
                <button type="submit">Compute GCD</button>
            </form>
            "#,
        )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        HttpResponse::BadRequest()
            .content_type("text/html; charset=utf-8")
            .body("Computing the GCD with zero is boring. Try again with positive integers.");
    }
    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>",
        form.n,
        form.m,
        handlers::gcd(form.n, form.m)
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(response)
}
