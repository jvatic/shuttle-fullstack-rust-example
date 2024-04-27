/* This build script ensure that everything needed to run the SquireCore server is in its place.
 * Primarily, this includes the static assets for the frontend, including the index, wasm app, and
 * JS bindings. Trunk is used to compile and generate the app and the JS bindings.
 */

use std::{env, process::Command};

fn main() -> Result<(), i32> {
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
        || env::var("PROFILE")
            .map(|v| v == "release")
            .unwrap_or_default()
    {
        compile_fe();
    }
    Ok(())
}

fn compile_fe() {
    // Calls trunk to compile the frontend
    let mut cmd = Command::new("trunk");
    cmd.args(["build", "-d", "../assets", "--public-url", "/assets"]);

    cmd.arg("--release");
    cmd.arg("../frontend/index.html");
    if let Err(e) = cmd.status() {
        panic!("Failed to compile frontend!\n{e}");
    }
}
