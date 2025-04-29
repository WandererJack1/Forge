// src/routes/pokemons/GOAT.rs
use TheMill_lib::{Request, Response};

#[TheMill_lib::handler]
async fn redirect_to_goat(_req: Request) -> Response {
    Response::Redirect("/pokemons/mewtwo".to_string())
}
