use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::{fs::{File, create_dir}, io::Write};
use subprocess::{Exec, Redirection};

#[derive(Debug, Serialize, Deserialize)]
struct UserCode {
    code: Vec<String>,
}

/// This handler uses json extractor with limit
async fn send_code(item: web::Json<UserCode>, req: HttpRequest) -> HttpResponse {

    /*
     * TODO: Add session handling in order to get 
     * proper usernames
     */
    let username = "test-user2";

    /*
     * Write code sent by the user to a file
     */
    let dir = "./usr/".to_string() + username;
    let volume = format!("{}{}", dir, ":/code:rw");
    print!("{}", dir);
    print!("{}", volume);
    create_dir(dir);
    let filepath = "./usr/".to_string() + username + "/code.py"; /* It's not done properly I think */
    let mut file = File::open(filepath);
    file.unwrap().write_all(item.code.join("\n").as_bytes());

    let out = Exec::cmd("podman")
        .arg("run")
        .arg("-v")
        .arg(volume)
        .stdout(Redirection::Pipe)
        .capture()
        .unwrap()
        .stdout_str();

    println!("STDOUT: {out}");

    HttpResponse::Ok().json(item.0) // <- send json response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/send_code").route(web::post().to(send_code)))
    })
    .bind(("192.168.0.100", 8080))?
    .run()
    .await
}
