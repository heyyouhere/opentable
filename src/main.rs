use serde::forward_to_deserialize_any;
use tiny_http::{Server, Request, Response, StatusCode, Method, Header};
use std::io::{Result, Read};
use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};

use crate::game::Game;



mod game;

fn serve_404(request: Request) -> Result<()>{
    request.respond(Response::from_string("404").with_status_code(StatusCode(404)))
}

fn serve_file(mut request: Request) -> Result<()>{
    let cors  = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();
    let cors2 = Header::from_bytes("Access-Control-Allow-Headers", "*").unwrap();
    let cors3 = Header::from_bytes("Access-Control-Allow-Methods", "*").unwrap();
    let mut filename = String::new();
    request.as_reader().read_to_string(&mut filename)?;
    println!("{filename}");
    let requested_file = File::open(filename)?;
    request.respond(Response::from_file(requested_file)
                            .with_header(cors)
                            .with_header(cors2)
                            .with_header(cors3)
                            .with_status_code(StatusCode(200)))
}

fn handle_request(mut request: Request) -> Result<()>{
    let cors  = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();
    let json  = Header::from_bytes("Content-type", "application/json").unwrap();
    let cors2 = Header::from_bytes("Access-Control-Allow-Headers", "*").unwrap();
    let cors3 = Header::from_bytes("Access-Control-Allow-Methods", "*").unwrap();
    match (request.method(), request.url()){
        (Method::Get, "/") => {
            request.respond(Response::from_file(File::open("./src/web-src/index.html")?)
                            .with_header(cors)
                            .with_header(cors2)
                            .with_header(cors3)
                            .with_status_code(StatusCode(200)))
        },
        (Method::Get, "/favicon.ico") => {
            request.respond(Response::from_file(File::open("./src/web-src/favicon.ico")?)
                            .with_header(cors)
                            .with_header(cors2)
                            .with_header(cors3)
                            .with_status_code(StatusCode(200)))
        },
        (Method::Get, "/style.css") => {
            request.respond(Response::from_file(File::open("./src/web-src/style.css")?)
                            .with_header(cors)
                            .with_header(cors2)
                            .with_header(cors3)
                            .with_status_code(StatusCode(200)))
        },
        (Method::Get, "/pdfobject.js") => {
            request.respond(Response::from_file(File::open("./src/web-src/pdfobject.js")?)
                            .with_header(cors)
                            .with_header(cors2)
                            .with_header(cors3)
                            .with_status_code(StatusCode(200)))
        },
        (Method::Get, "/api/games") => {
            let games = game::load_games("./src/games/");
            request.respond(Response::from_string(serde_json::to_string(&games).unwrap())
                          .with_header(json)
                            .with_header(cors)
                            .with_header(cors2)
                            .with_header(cors3)
                            .with_status_code(StatusCode(200))
                            )
        }
        (Method::Post, "/api/games") => {
            let mut content = String::new();
            request.as_reader().read_to_string(&mut content)?;
            let new_game = game::Game::new(content);
            new_game.save_local();
            request.respond(Response::from_string("Created New game")
                            .with_header(cors)
                            .with_header(cors2)
                            .with_header(cors3)
                            .with_status_code(StatusCode(200)))

        }
        (Method::Get, "/api/pdfs") => {
            let paths = read_dir("src/assets/").unwrap();
            let mut j = "[".to_string();
            for p in paths{
                j = format!("{j}\"{}\",", p.unwrap().path().to_str().unwrap().to_string());
            }
            j.pop();
            j = format!("{j}]");
            request.respond(Response::from_string(j)
                            .with_header(json)
                            .with_header(cors)
                            .with_header(cors2)
                            .with_header(cors3)
                            .with_status_code(StatusCode(200)))
        },
        (Method::Get, url) if url.starts_with("/assets")=>
        {
            let url = request.url();
            let path = Path::new("./src/").join(url.strip_prefix('/').unwrap());
            if let Ok(mut file) = File::open(path) {
                let mut content = Vec::new();
                file.read_to_end(&mut content)?;
                let response = Response::from_data(content);
                let pdf1 = Header::from_bytes("Content-Type", "application/pdf").unwrap();
                request.respond(response
                                .with_header(cors)
                                .with_header(cors2)
                                .with_header(cors3)
                                .with_header(pdf1))

            } else {
                request.respond(Response::from_string("404 Not Found"))
            }
        },
        (Method::Options, _) =>  {
            request.respond(Response::from_string("")
                .with_header(cors)
                .with_header(cors2)
                .with_header(cors3)
                .with_status_code(StatusCode(200)))
        },
        _ => serve_404(request)

    }
}

fn main(){
    let address = "0.0.0.0:1583";
    let server = Server::http(&address).unwrap();

    for request in server.incoming_requests(){
        println!("{request:?}");
        let _ = handle_request(request).map_err(|err| 
                                                eprintln!("[ERROR] While handling request error happend: {err}"));
    }
}




