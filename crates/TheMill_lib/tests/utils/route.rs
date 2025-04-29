use TheMill_lib::{Props, Request, Response};

#[TheMill_lib::handler]
async fn route(_: Request) -> Response {
    Response::Props(Props::new("{}"))
}
