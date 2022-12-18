use std::path::PathBuf;


use actix_multipart::Multipart;
use actix_files::Files;
use actix_web::*;
use actix_web::HttpRequest;
use actix_web::{get, post,web, App, HttpResponse, HttpServer, Responder, web::Data};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Write, BufReader, BufRead, Error};
use calamine::{Reader, Xlsx, open_workbook};
use serde::Serialize;


const HOST:(&str,u16) = ("127.0.0.1",8082);

#[derive(Serialize)]
struct UserData {
    id:i16
}



fn next_filename() -> Option<PathBuf> {
    let mut p = PathBuf::new();
    let time:String = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string();
    p.push(format!("uploaded-excel/{}.xlsx",time));
    Some(p)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("200")
}

#[post("/trance")]
async fn trance_ex_to_js(req_excel: HttpRequest) -> web::Json<UserData> {

    // let mut excel:Xlsx<_> = req_excel;
    //엑셀로 받아오면 됨 ㅇㅇ

    println!("{:?}",req_excel);
    
    
    web::Json(UserData { id:1 })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running at http://{}:{}",HOST.0,HOST.1);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(trance_ex_to_js)
    })
    .bind(HOST)?
    .run()
    .await
}