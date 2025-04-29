use TheMill_lib::Request;
use TheMill_lib::axum::http::StatusCode;

#[TheMill_lib::api(GET)]
pub async fn health_check(_req: Request) -> StatusCode {
    StatusCode::OK
}
