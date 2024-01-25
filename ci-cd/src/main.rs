use std::process::Stdio;

use axum::{extract::{Request, Json, Path, Extension, Query}, Router, routing::post};
use serde_json::Value;

use serde::{Deserialize, Serialize};
use tokio::{fs::canonicalize, process::Command};

#[derive(Debug, Serialize, Deserialize)]
struct Author {
    name: String,
    email: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Committer {
    name: String,
    email: String,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HeadCommit {
    id: String,
    tree_id: String,
    distinct: bool,
    message: String,
    timestamp: String,
    url: String,
    author: Author,
    committer: Committer,
    added: Vec<String>,
    removed: Vec<String>,
    modified: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PushPayload {
    head_commit: HeadCommit,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", post(handle_push));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7777").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn handle_push(Json(payload): Json<Value>) {
    if let Ok(push_payload) = serde_json::from_value::<PushPayload>(payload) {

        if push_payload.head_commit.author.name == "niooii" {

            println!("AWFWAFWA wtf man {:?}", canonicalize("../").await.unwrap());

            let mut docker_down = Command::new("docker");
            docker_down.args(["compose", "down"])
            .current_dir(canonicalize("../").await.unwrap());

            let mut docker_build = Command::new("docker");
            docker_build.args(["compose", "build"])
            .current_dir(canonicalize("../").await.unwrap());

            let mut docker_up = Command::new("docker");
            docker_up.args(["compose", "up", "-d"])
            .current_dir(canonicalize("../").await.unwrap());

            let commands = [docker_down, docker_build, docker_up];

            for mut command in commands {
                let command_out = command.spawn().expect("Could not spawn process")
                .wait_with_output().await.expect("failed to execute command").stdout;
                
                let stdout = String::from_utf8_lossy(&command_out);
                
                println!("{}", stdout);
            }
        }

        println!("finish");

    } else {
        println!("There was an error parsing the payload json...");
    }
}