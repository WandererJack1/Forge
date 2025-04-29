use assert_cmd::Command;
use serial_test::serial;
use std::fs;
use tracing::Level;

mod utils;
use utils::temp_TheMill_project::TempTheMillProject;

const POST_API_FILE: &str = r"#[TheMill_lib::api(POST)]";
const GET_API_FILE: &str = r"#[TheMill_lib::api(GET)]";

fn tracing_message(level: Level, module: &str, message: &str) -> String {
    format!("\x1b[31m{level}\x1b[0m \x1b[2mTheMill::{module}\x1b[0m\x1b[2m:\x1b[0m {message}\n")
}

#[cfg(target_os = "windows")]
const BUILD_TheMill_CONFIG: &str = ".\\node_modules\\.bin\\TheMill-build-config.cmd";

#[cfg(not(target_os = "windows"))]
const BUILD_TheMill_CONFIG: &str = "./node_modules/.bin/TheMill-build-config";

#[test]
#[serial]
fn it_successfully_create_the_index_route() {
    let temp_TheMill_project = TempTheMillProject::new();

    temp_TheMill_project.add_file("./src/routes/index.rs");

    let mut test_TheMill_build = Command::cargo_bin("TheMill").unwrap();
    test_TheMill_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_main_rs_path = temp_TheMill_project.path().join(".TheMill/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.TheMill/main.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/index.rs"]"#));
    assert!(temp_main_rs_content.contains("mod index;"));

    assert!(temp_main_rs_content
        .contains(r#".route("/", get(index::TheMill_internal_route)).route("/__TheMill/data/", get(index::TheMill_internal_api))"#));
}

#[test]
#[serial]
fn it_successfully_create_an_api_route() {
    let temp_TheMill_project = TempTheMillProject::new();

    temp_TheMill_project.add_file_with_content("./src/routes/api/health_check.rs", POST_API_FILE);

    let mut test_TheMill_build = Command::cargo_bin("TheMill").unwrap();
    test_TheMill_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_main_rs_path = temp_TheMill_project.path().join(".TheMill/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.TheMill/main.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/api/health_check.rs"]"#));
    assert!(temp_main_rs_content.contains("mod api_health_check;"));

    assert!(temp_main_rs_content.contains(
        r#".route("/api/health_check", post(api_health_check::post_TheMill_internal_api))"#
    ));
}

#[test]
#[serial]
fn it_successfully_create_multiple_api_for_the_same_file() {
    let temp_TheMill_project = TempTheMillProject::new();

    temp_TheMill_project.add_file_with_content(
        "./src/routes/api/health_check.rs",
        &format!("{POST_API_FILE}{GET_API_FILE}"),
    );

    let mut test_TheMill_build = Command::cargo_bin("TheMill").unwrap();
    test_TheMill_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_main_rs_path = temp_TheMill_project.path().join(".TheMill/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.TheMill/main.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/api/health_check.rs"]"#));
    assert!(temp_main_rs_content.contains("mod api_health_check;"));

    assert!(temp_main_rs_content.contains(
        r#".route("/api/health_check", post(api_health_check::post_TheMill_internal_api))"#
    ));
    assert!(
        temp_main_rs_content.contains(
            r#".route("/api/health_check", get(api_health_check::get_TheMill_internal_api))"#
        )
    );
}

#[test]
#[serial]
fn it_successfully_create_catch_all_routes() {
    let temp_TheMill_project = TempTheMillProject::new();

    temp_TheMill_project.add_file("./src/routes/[...all_routes].rs");

    temp_TheMill_project.add_file_with_content("./src/routes/api/[...all_apis].rs", POST_API_FILE);

    let mut test_TheMill_build = Command::cargo_bin("TheMill").unwrap();
    test_TheMill_build
        .arg("build")
        .arg("--no-js-emit")
        .assert()
        .success();

    let temp_main_rs_path = temp_TheMill_project.path().join(".TheMill/main.rs");

    let temp_main_rs_content =
        fs::read_to_string(&temp_main_rs_path).expect("Failed to read '.TheMill/main.rs' content.");

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/api/[...all_apis].rs"]"#));
    assert!(temp_main_rs_content.contains("mod api_dyn_catch_all_all_apis;"));

    assert!(temp_main_rs_content.contains(r#"#[path="../src/routes/[...all_routes].rs"]"#));
    assert!(temp_main_rs_content.contains("mod dyn_catch_all_all_routes;"));

    assert!(temp_main_rs_content.contains(
        r#".route("/api/{*all_apis}", post(api_dyn_catch_all_all_apis::post_TheMill_internal_api))"#
    ));

    assert!(temp_main_rs_content.contains(
        r#".route("/{*all_routes}", get(dyn_catch_all_all_routes::TheMill_internal_route))"#
    ));

    assert!(temp_main_rs_content.contains(
        r#".route("/{*all_routes}", get(dyn_catch_all_all_routes::TheMill_internal_route))"#
    ));

    assert!(temp_main_rs_content.contains(
        r#".route("/__TheMill/data/{*all_routes}", get(dyn_catch_all_all_routes::TheMill_internal_api))"#
    ));
}

#[test]
#[serial]
fn it_fails_without_installed_build_config_script() {
    let _guard = TempTheMillProject::new();

    let mut test_TheMill_build = Command::cargo_bin("TheMill").unwrap();
    test_TheMill_build
        .arg("build")
        .assert()
        .failure()
        .stderr("Failed to find the build script. Please run `npm install`\n");
}

#[test]
#[serial]
fn it_fails_without_installed_build_script() {
    let temp_TheMill_project = TempTheMillProject::new();
    temp_TheMill_project.add_file_with_content(BUILD_TheMill_CONFIG, "#!/bin/bash");
    Command::new("chmod")
        .arg("+x")
        .arg(BUILD_TheMill_CONFIG)
        .assert()
        .success();
    let mut test_TheMill_build = Command::cargo_bin("TheMill").unwrap();

    test_TheMill_build
        .arg("build")
        .assert()
        .failure()
        .stderr("Failed to read config. Please run `npm install` to generate automatically.\n");
}

#[test]
#[serial]
fn dev_fails_with_no_config() {
    let _temp_TheMill_project = TempTheMillProject::new_with_no_config();

    let mut test_TheMill_build = Command::cargo_bin("TheMill").unwrap();
    test_TheMill_build
        .arg("dev")
        .assert()
        .failure()
        .stdout(tracing_message(
            Level::ERROR,
            "source_builder",
            "Cannot find TheMill.config.ts - is this a TheMill project?",
        ));
}

#[test]
#[serial]
fn build_fails_with_no_config() {
    let _temp_TheMill_project = TempTheMillProject::new_with_no_config();

    let mut test_TheMill_build = Command::cargo_bin("TheMill").unwrap();
    test_TheMill_build
        .arg("dev")
        .assert()
        .failure()
        .stdout(tracing_message(
            Level::ERROR,
            "source_builder",
            "Cannot find TheMill.config.ts - is this a TheMill project?",
        ));
}
