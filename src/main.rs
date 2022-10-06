use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::{fs::{OpenOptions, create_dir}, io::{Write, ErrorKind}};
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

#[derive(Debug, Serialize, Deserialize)]
struct ErrorOutput {
    code: i32,
    error: String,
}

impl ErrorOutput {
    pub fn new(code :i32,error :&str) -> ErrorOutput {
        return ErrorOutput { code: code, error: String::from(error) }
    }
}

/// This handler uses json extractor with limit
async fn send_code(item: web::Json<UserCode>, _: HttpRequest) -> HttpResponse {

    /*
     * TODO: Add session handling in order to get 
     * proper usernames
     */
    let username = "testuser";

    /*
     * Create path to users directory where his code will be stored
     * and create argument string for a volume that is passed to
     * podman-run. It has to be ":ro" (i.e read-only), so the user
     * is not able to create any files in directory shared between
     * host and users container. 
     */
    let dir = "./usr/".to_string() + username;
    let volume = format!("{}{}", dir, ":/code:ro");

    /*
     * Create directory for users code
     * TODO: check if it already exists
     */
    let create_dir_result = create_dir(dir);
    match create_dir_result {
        Ok(_) => (),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists=> (),
            _ => {
                return HttpResponse::Ok().json(ErrorOutput::new(-1,"Could not create users directory"));
            }
        }
    }

    /*
     * Open or create file for users code
     */
    let filepath = "./usr/".to_string() + username + "/code.py"; /* It's not done properly I think */
    let file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filepath);
    let mut file = match file_result {
        Ok(file) => file,
        Err(_) => return HttpResponse::Ok().json(ErrorOutput::new(-2,"Could not open file with users code")),
    };

    /*
     * Clear all the contents of the file with users code 
     */
    let truncate_result = file.set_len(0);
    match truncate_result {
        Ok(_) => (),
        Err(_) => return HttpResponse::Ok().json(ErrorOutput::new(-3,"Could not clear contents of users file")),
    };

    /*
     * Write code sent by a user into the file in his directory.
     * Code is sent as an array of strings, so we must join it
     * with newline character between before writing to the file.
     */
    let write_result = file.write_all(item.code.join("\n").as_bytes());
    match write_result {
        Ok(_) => (),
        Err(_) => return HttpResponse::Ok().json(ErrorOutput::new(-4,"Could not write users code into users file")),
    };

    /*
     * Execute users code in a safe podman container, the containers are set to timeout
     * after 2 seconds. I wanted to limit their memory, but I had issues with doing that
     * in a rootless container. Code sent by the user can be found in directory /usr/username
     * and is binded to the container using "-v" option, so the container can read it's contents
     * and execute it.
     * TODO: Limit containters memory
     */
    let process_result = Exec::cmd("podman")
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
        .capture();
    let process = match process_result {
        Ok(process) => process,
        Err(_) => return HttpResponse::Ok().json(ErrorOutput::new(-5,"Could not run users container")),
    };

    let stdout = process.stdout_str();
    let stderr = process.stdout_str();
    println!("STDOUT:\n{stdout}");
    println!("STDERR:\n{stdout}");

    /*
     * We return the code as a list of strings, so we must split it
     * by the newline character.
     */
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
