use TheMill_lib::Request;

#[TheMill_lib::api(GET)]
async fn read_dynamic_parameter(req: Request) -> String {
    let param = req
        .params
        .get("parameter")
        .expect("Failed to get the catch_all param");

    param.to_string()
}
