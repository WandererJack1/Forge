// File automatically generated
// Do not manually change it

use TheMill_lib::{tokio, Mode, Server, axum::Router, TheMill_internal_init_v8_platform};
// AXUM_GET_ROUTE_HANDLER

const MODE: Mode = /*MODE*/;

// MODULE_IMPORTS

//MAIN_FILE_IMPORT//

#[tokio::main]
async fn main() {
    TheMill_internal_init_v8_platform();
    
    if MODE == Mode::Prod {
        println!("\n  ⚡ TheMill v/*VERSION*/");
    }

    //MAIN_FILE_DEFINITION//

    let router = Router::new()
        // ROUTE_BUILDER
        //MAIN_FILE_USAGE//;

    Server::init(router, MODE).await.start().await
}

