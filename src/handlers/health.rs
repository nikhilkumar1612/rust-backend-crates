use actix_web::{HttpRequest, HttpResponse, get, post, web};
use reqwest::Client;

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

#[post("/redirect")]
pub async fn redirect(
    req: HttpRequest, body: web::Bytes
) -> HttpResponse {
    let client = Client::new();
    let mut upstream = client.post("https://eth.llamarpc.com");

    for (k, v) in req.headers() {
        if let Ok(val) = v.to_str() {
            upstream = upstream.header(k.as_str(), val);
        }
    }

    let resp = upstream.body(body).send().await.unwrap();
    let status = resp.status().as_u16();
    let headers = resp.headers().clone();
    let bytes = resp.bytes().await.unwrap();

    let mut response = HttpResponse::build(
        actix_web::http::StatusCode::from_u16(status).unwrap()
    );

    for (k, v) in headers {
        if let Some(name) = k {
            if let Ok(val) = v.to_str() {
                response.insert_header((name.as_str(), val));
            }
        }
    }

    response.body(bytes)
}
