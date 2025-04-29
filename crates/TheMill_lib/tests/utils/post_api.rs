use serde::Deserialize;
use TheMill_lib::Request;

#[derive(Deserialize)]
struct Payload {
    data: String,
}

#[TheMill_lib::api(POST)]
async fn health_check(req: Request) -> String {
    req.body::<Payload>().unwrap().data
}
