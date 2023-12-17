use actix_web::{get, HttpResponse, web::Path};

#[get("/1/{input_1}/{inputs:.*}")]
async fn xor_pow_endpoint(path: Path<(i32,String)>) -> HttpResponse {
    let (input1,others) = path.into_inner();
    let remaining_inputs = others.split('/').map(|item| item.parse::<i32>().unwrap_or_default()).collect::<Vec<i32>>();
    let mut response_xor = input1;

    for item in remaining_inputs{
        response_xor = response_xor ^ item;
    }
    let powed_input = response_xor.pow(3);
    HttpResponse::Ok().json(powed_input)
}
