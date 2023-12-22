use actix_web::{web::{Json, ServiceConfig},  HttpResponse, post};
use serde::Deserialize;

pub fn day14_routes(cfg: &mut ServiceConfig) {
    cfg.service(unsafe_html);
    cfg.service(safe_html);
}

#[derive(Deserialize)]
struct Content{
    content:String
}

const HTML_TEMPLATE:&str =
"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {{ template }}
  </body>
</html>";


#[post("/14/unsafe")]
async fn unsafe_html(Json(html):Json<Content>) ->  HttpResponse {

    let response = HTML_TEMPLATE.replace("{{ template }}", &html.content);
    HttpResponse::Ok().body(response)
}

#[post("/14/safe")]
async fn safe_html(Json(html):Json<Content>) ->  HttpResponse {

    let response = HTML_TEMPLATE.replace("{{ template }}",&handlebars::html_escape(&html.content));
    HttpResponse::Ok().body(response)
}