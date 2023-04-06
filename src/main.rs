

use actix_web::{get, post, web, Error, HttpResponse, Responder, HttpServer, App};
use actix_web_lab::respond::Html;
use juniper::http::{playground::playground_source, GraphQLRequest};
use schema::create_schema;

use crate::schema::Schema;

mod schema;

#[post("/graphql")]
pub async fn graphql(
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = {};

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/graphql")]
async fn graphql_playground() -> impl Responder {
    Html(create_playground())
}

pub fn create_playground() -> String  {
    playground_source("/graphql", None)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(create_schema()))
            .service(graphql_playground)
            .service(graphql)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}