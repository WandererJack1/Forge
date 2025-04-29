use serde::Serialize;
use TheMill_lib::{Props, Request, Response, Type};

#[derive(Serialize, Type)]
struct MyResponse<'a> {
    subtitle: &'a str,
}

#[TheMill_lib::handler]
async fn get_server_side_props(_req: Request) -> Response {
    Response::Props(Props::new(MyResponse {
        subtitle: "Subtitle received from the server",
    }))
}
