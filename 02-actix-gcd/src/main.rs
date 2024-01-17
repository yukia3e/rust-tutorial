use std::mem::swap;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
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
#[post("/gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Invalid input");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_index).service(post_gcd))
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
}
