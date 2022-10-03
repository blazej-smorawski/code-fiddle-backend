use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::{fs::{OpenOptions, create_dir}, io::Write};
use subprocess::{Exec, Redirection};

#[derive(Debug, Serialize, Deserialize)]
struct UserCode {
    code: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CodeOutput {
    stdout: Vec<String>,
    stderr: Vec<String>,
}

/// This handler uses json extractor with limit
async fn send_code(item: web::Json<UserCode>, req: HttpRequest) -> HttpResponse {

    /*
     * TODO: Add session handling in order to get 
     * proper usernames
     */
    let username = "testuser";

    /*
     * Write code sent by the user to a file
     */
    let dir = "./usr/".to_string() + username;
    let volume = format!("{}{}", dir, ":/code:r");
    create_dir(dir);

    let filepath = "./usr/".to_string() + username + "/code.py"; /* It's not done properly I think */
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filepath);

    let mut f2 = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    f2.set_len(0);
    f2.write_all(item.code.join("\n").as_bytes());

    let process = Exec::cmd("podman")
        .arg("run")
        //.arg("-m")
        //.arg("256m")
        .arg("--timeout")
        .arg("2")
        .arg("-v")
        .arg(volume)
        .arg("code-fiddle-python-3.10")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .unwrap();


    let stdout = process.stdout_str();
    let stderr = process.stdout_str();
    println!("STDOUT:\n{stdout}");
    println!("STDERR:\n{stdout}");

    let output = CodeOutput {
        stdout: stdout.split("\n").map(str::to_string).collect(),
        stderr: stderr.split("\n").map(str::to_string).collect(),
    };

    HttpResponse::Ok().json(output) // <- send json response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(web::resource("/send_code").route(web::post().to(send_code)))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
