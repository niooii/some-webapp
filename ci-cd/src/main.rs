use axum::{extract::Json, Router, routing::post};
use serde_json::Value;

use serde::{Deserialize, Serialize};
use std::{process::Command};
use std::time::Duration;
use wait_timeout::ChildExt;
use std::process::{Child, ExitStatus};

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
    
    par_loop(10).await;

    let app = Router::new()
    .route("/", post(handle_push));

    let address = "0.0.0.0:7777";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Now listening for github webhooks on {address}");

    axum::serve(listener, app).await.unwrap();

}

async fn handle_push(Json(payload): Json<Value>) {

    println!("Recieved push payload");

    if let Ok(push_payload) = serde_json::from_value::<PushPayload>(payload) {

        if push_payload.head_commit.author.name == "niooii" {
            
           par_loop(10).await;

            println!("Finish");

        } else {
            println!("Invalid head commit author.");
        }
    } else {
        println!("There was an error parsing the payload json...");
    }
}

async fn par_loop(max_retry: u16) {
    let mut retry = 0_u16;
    loop {
        let result = pull_and_restart(40.0).await;

        if result.is_ok() {
            break;
        } else {
            if retry == max_retry {
                break;
            }
            println!("\nRetrying ({retry} of {max_retry})...\n");
            retry+=1;
        }
    }
}

async fn pull_and_restart(timeout_secs: f32) -> Result<(), String> {
    let mut docker_down = Command::new("docker");
    docker_down.args(["compose", "down"])
    .current_dir("../");

    let mut git_pull = Command::new("git");
    git_pull.arg("pull")
    .current_dir("../");

    let mut docker_build = Command::new("docker");
    docker_build.args(["compose", "build"])
    .current_dir("../");

    let mut docker_up = Command::new("docker");
    docker_up.args(["compose", "up", "-d"])
    .current_dir("../");

    let commands = [docker_down, git_pull, docker_build, docker_up];
    
    let mut i = 1_u16;
    let num_commands = commands.iter().len() as u16;
    for mut command in commands {

        let mut child = command.spawn().unwrap();

        match child.wait_timeout(Duration::from_secs_f32(timeout_secs)) {
            Ok(_) => {
                println!("\nFinished executing command {i} of {num_commands}\n");
            }
            Err(_) => {
                child.kill().unwrap();
                println!("{}", format!("Process has been executing for {timeout_secs} seconds, appears to be hung. Exiting early..."));
                return Err("Process took too long.".into());
            }
        };
        
        i += 1;

    }

    Ok(())
}