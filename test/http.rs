use actix_web::{get, guard::Guard, App, HttpRequest, HttpResponse, HttpServer, Responder};
use base64::prelude::*;

pub struct User<'a>{
    username: &'a str,
    password: &'a str
}

impl<'a> Guard for User<'a> {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        println!("{:?}", ctx);
        if ctx.head().headers().get("Username").unwrap() == "test123" {
            return ctx.head().headers().get("Password").unwrap() == "test123"
        } else {
            false
        }
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is the Verdete's test HTTP server")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world")
}

#[get("/auth")]
async fn basic_auth(payload: HttpRequest) -> impl Responder {
    match payload.headers().get("authorization"){
        Some(user) => {
            let parsed = user.to_str().unwrap().replace("Basic ", "");
            let decoded = BASE64_STANDARD.decode(parsed).unwrap();
            let usr: Vec<&str> = std::str::from_utf8(decoded.as_slice()).unwrap().split(":").collect();
            if usr[0] == "test123" && usr[1] == "test123" {
                HttpResponse::Ok().body("Hello, world")
            } else {
                HttpResponse::Unauthorized().await.unwrap()
            }
        },
        None => HttpResponse::Unauthorized().append_header(("WWW-Authenticate", "Basic realm='Verdete'")).await.unwrap()
    }
}

#[get("/token")]
async fn token_auth(payload: HttpRequest) -> impl Responder {
    match payload.headers().get("authorization") {
        Some(user) => {
            let usr = user.to_str().unwrap().replace("Bearer ", "");
            if usr == "123123" {
                return HttpResponse::Ok().body("Hello, world")
            }
            HttpResponse::Unauthorized().await.unwrap()
        },
        None => HttpResponse::Unauthorized().append_header(("WWW-Authenticate", "Bearer")).await.unwrap()
    }
}

#[get("/super_secret")]
async fn x509_auth(payload: HttpRequest) -> impl Responder {
    println!("{:?}", payload.headers().get("authorization"));
    HttpResponse::Unauthorized().await.unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
    App::new()
        .service(index)
        .service(hello)
        .service(basic_auth)
        .service(token_auth)
        .service(x509_auth)
    })
    .bind(("127.0.0.1", 8000))
    .unwrap()
    .run()
    .await
}