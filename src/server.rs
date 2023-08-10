use tiny_http::{Server, Request, Response, StatusCode, Method, Header};
use std::io::Result;

fn serve_404(request: Request) -> Result<()>{
    request.respond(Response::from_string("404").with_status_code(StatusCode(404)))
}




fn handle_request(request: Request) -> Result<()>{
    let cors  = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();
    let cors2 = Header::from_bytes("Access-Control-Allow-Headers", "*").unwrap();
    let cors3 = Header::from_bytes("Access-Control-Allow-Methods", "*").unwrap();
    match (request.method(), request.url()){
        (Method::Get, "/") => {
            let response = Response::from_string("[{\"hello\" : \"world\"}, {\"you\" : \"sexy\"}]")
                .with_header(cors)
                .with_header(cors2)
                .with_header(cors3)
                .with_status_code(StatusCode(200));
            request.respond(response)
        },
        (Method::Get, "/api/get_games") => request.respond(Response::from_string("You made a get request").with_status_code(StatusCode(200))),
        (Method::Post, "/api/post") => request.respond(Response::from_string("You made a get request").with_status_code(StatusCode(200))),
        (Method::Options, "/") =>  {
                let response = Response::from_string("")
                    .with_header(cors)
                .with_header(cors2)
                .with_header(cors3)
                .with_status_code(StatusCode(200));
            request.respond(response)
        },
        _ => serve_404(request)

    }
}

pub fn serve(address : &str){
    let server = Server::http(&address).unwrap();

    for request in server.incoming_requests(){
        println!("{request:?}");
        let _ = handle_request(request).map_err(|err| 
                                                eprintln!("[ERROR] While handling request error happend: {err}"));
    }
}




