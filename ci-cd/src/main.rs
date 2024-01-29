mod webhook_notifier;

use axum::{extract::Json, Router, routing::post};
use serde_json::Value;

use serde::{Deserialize, Serialize};
use tokio::time;
use std::{process::Command};
use std::time::Duration;
use wait_timeout::ChildExt;
use std::process::{Child, ExitStatus};

use crate::webhook_notifier::{Notifier, Stage};

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
    
    let notifier = Notifier::new("https://discordapp.com/api/webhooks/1200995793310593136/JsRTQ0yetL5VowCnQ3t3k4tVujmkDYqjiXynOwftU_mi_3pWZWimn5EBlXNo0pommYdz").await
    .unwrap();

    par_loop(10, &notifier).await.expect("Reached max retry attempts. Exiting...");



    let app = Router::new()
    .route("/", post(handle_push));

    let address = "0.0.0.0:7777";

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("\nListening for github webhooks on {address}.\n");

    axum::serve(listener, app).await.unwrap();

}

async fn handle_push(Json(payload): Json<Value>) {

    let notifier = Notifier::new("https://discordapp.com/api/webhooks/1200995793310593136/JsRTQ0yetL5VowCnQ3t3k4tVujmkDYqjiXynOwftU_mi_3pWZWimn5EBlXNo0pommYdz").await
        .unwrap();

    println!("\nRecieved push payload.\n");

    notifier.send(Stage::PushDetected, "@here").await.unwrap();

    let start = time::Instant::now();

    if let Ok(push_payload) = serde_json::from_value::<PushPayload>(payload) {
        // push_payload.head_commit.author.name == "niooii"
        if true {

            match par_loop(10, &notifier).await {
                Ok(_try) => {
                    println!("Command sequence finished executing successfully ({_try} tries.)");
                },
                Err(e) => {
                    println!("{e}");
                },
            }
            
            println!("\nExecution finished in {:.2} seconds.\n", start.elapsed().as_secs_f32());

        } else {
            println!("Invalid head commit author.");
        }
    } else {
        println!("There was an error parsing the payload json...");
    }
}

async fn par_loop(max_retry: u16, notifier: &Notifier) -> Result<u16, String> {
    let mut _try = 1_u16;

    loop {
        
        notifier.send(Stage::Reloading { _try }, "@here").await.unwrap();

        let result = pull_and_restart(600.0).await;

        if result.is_ok() {
            notifier.send(Stage::Finish { _try }, "@here").await.unwrap();
            break;
        } else {
            _try += 1;
            if _try - 1 == max_retry {
                let fail_string = format!("Retry limit reached ({max_retry}).");
                notifier.send(Stage::Fail, format!("@here\n{fail_string}").as_str()).await.unwrap();
                return Err(fail_string);
            }
            println!("\nRetrying ({} of {max_retry})...\n", _try - 1);
        }
    }

    Ok(_try)
}

async fn pull_and_restart(timeout_secs: f32) -> Result<(), String> {
    let mut docker_down = Command::new("docker");
    docker_down.args(["compose", "down"])
    .current_dir("../");

    let mut stop_niooi_backend = Command::new("docker");
    stop_niooi_backend.args(["stop", "niooi_backend"])
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

    let mut build_niooi_backend = Command::new("docker");
    build_niooi_backend.args(["build", "--tag", "--name=niooi_backend", "-d", "niooi_backend"])
    .current_dir("../backend");

    let mut run_niooi_backend = Command::new("docker");
    run_niooi_backend.args(["run", "--net=host", "'niooi_backend'", ".", "--network=host"])
    .current_dir("../backend");

    let commands = [docker_down, stop_niooi_backend, git_pull, docker_build, docker_up, build_niooi_backend, run_niooi_backend];
    
    let mut i = 1_u16;
    let num_commands = commands.iter().len() as u16;
    for mut command in commands {

        let mut child = command.spawn().unwrap();

        match child.wait_timeout(Duration::from_secs_f32(timeout_secs)) {
            Ok(_) => {
                println!("\nFinished executing command {i} of {num_commands}.\n");
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