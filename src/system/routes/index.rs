use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use sqlx::MySqlPool;


#[get("/")]
async fn index(db_pool: web::Data<MySqlPool>) -> impl Responder {
    let result = "Hello World";
    let response: HttpResponse = HttpResponse::Ok().content_type("text/html").body(result);
    response
}