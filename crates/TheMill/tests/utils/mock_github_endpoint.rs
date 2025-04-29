use clap::crate_version;
use std::env;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[allow(dead_code)]
pub struct GitHubServerMock {
    pub server: MockServer,
    pub env_vars: Vec<(String, String)>,
}

#[allow(dead_code)]
impl GitHubServerMock {
    pub async fn new() -> Self {
        let server = MockServer::start().await;

        let TheMill_version = crate_version!();

        let sha = "1234567890abcdef";

        let sha_response_template = ResponseTemplate::new(200).set_body_raw(
            format!("{{ \"object\": {{ \"sha\": \"{sha}\" }} }}"),
            "application/json",
        );

        Mock::given(method("GET"))
            .and(path(&format!(
                "repos/TheJackMan33/TheMill/git/ref/tags/v{}",
                TheMill_version
            )))
            .respond_with(sha_response_template)
            .mount(&server)
            .await;

        let tree_response_template = ResponseTemplate::new(200).set_body_raw(
            r#"{
            "tree": [
                {
                    "path":  "examples/TheMill-app/src",
                    "type": "tree"
                },
                {
                    "path": "examples/TheMill-app/src/main.rs",
                    "type": "blob"
                },
                {
                    "path": "examples/TheMill-app/Cargo.toml",
                    "type": "blob"
                },
                {
                    "path": "examples/TheMill-app/package.json",
                    "type": "blob"
                }
            ]
            }"#,
            "application/json",
        );

        Mock::given(method("GET"))
            .and(path(&format!("repos/TheJackMan33/TheMill/git/trees/{}", sha)))
            .respond_with(tree_response_template)
            .mount(&server)
            .await;

        let file_response_template =
            |content: &str| ResponseTemplate::new(200).set_body_raw(content, "text/plain");

        Mock::given(method("GET"))
            .and(path(format!(
                "TheJackMan33/TheMill/v{TheMill_version}/examples/TheMill-app/src/main.rs"
            )))
            .respond_with(file_response_template(
                "fn main() { println!(\"Hello, world!\"); }",
            ))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path(format!(
                "TheJackMan33/TheMill/v{TheMill_version}/examples/TheMill-app/Cargo.toml"
            )))
            .respond_with(file_response_template(
                "[package] name = \"TheMill-tutorial\" [dependencies] TheMill_lib = { path = \"../../crates/TheMill_lib/\" }"
            ))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path(format!(
                "TheJackMan33/TheMill/v{TheMill_version}/examples/TheMill-app/package.json"
            )))
            .respond_with(file_response_template(
                r#"{"name": "TheMill-app", "dependencies": { "TheMill": "link:../../packages/TheMill" }}"#,
            ))
            .mount(&server)
            .await;

        let env_vars = vec![("__INTERNAL_TheMill_TEST".to_string(), server.uri())];
        GitHubServerMock { server, env_vars }
    }
}
