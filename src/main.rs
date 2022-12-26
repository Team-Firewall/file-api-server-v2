
use actix_cors::Cors;
use actix_easy_multipart::MultipartForm;
use actix_easy_multipart::tempfile::Tempfile;
use actix_web::{get, post,web, App, HttpResponse, HttpServer, Responder, http};
use calamine::{Reader, Xlsx, open_workbook};
use serde::Serialize;


const HOST:(&str,u16) = ("127.0.0.1",8082);

#[derive(Serialize)]
struct UserData {
    position:i16,
    name:String,
    grade:Option<i8>,
    class:Option<i8>,
    number:Option<i8>,
    phone:String,
    m_phone1:Option<String>,
    m_phone2:Option<String>,
    id:String,
    password:String
}

#[derive(Serialize)]
struct Regulation {
    checked:String,
    division:String,
    regulate:String,
    score:Option<i32>
}

#[derive(MultipartForm)]
struct Form {
    file_set: Option<Tempfile>,
}

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("
        api 목록\n
        /trance-student [POST]\n
        /trance-regulate [POST]
    ")
}

#[post("/trance-student")]
async fn trance_student_ex_to_js(req_excel: MultipartForm<Form>) -> web::Json<Vec<UserData>> {

    let mut excel:Xlsx<_> = open_workbook(req_excel.file_set.as_ref().unwrap().file.path()).unwrap();
    let mut result:Vec<UserData> = Vec::new();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            // println!("row={:?}", row);
            result.push(UserData {
                position: row[0].get_float().unwrap() as i16,
                name:row[1].to_string(),
                grade:Some(row[2].get_float().unwrap() as i8),
                class:Some(row[3].get_float().unwrap() as i8),
                number:Some(row[4].get_float().unwrap() as i8),
                phone:row[5].to_string(),
                m_phone1:Some(row[6].to_string()),
                m_phone2:Some(row[7].to_string()),
                id:row[8].to_string(),
                password:row[9].to_string()
            });
        }
    }
    
    web::Json(result)
}

#[post("/trance-regulate")]
async fn trance_regulate_ex_to_js(req_excel: MultipartForm<Form>) -> web::Json<Vec<Regulation>> {

    let mut excel:Xlsx<_> = open_workbook(req_excel.file_set.as_ref().unwrap().file.path()).unwrap();
    let mut result:Vec<Regulation> = Vec::new();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            // println!("row={:?}", row);
            result.push(Regulation {
                checked:row[0].to_string(),
                division:row[1].to_string(),
                regulate:row[2].to_string(),
                score:Some(row[3].get_float().unwrap() as i32)
            });
        }
    }
    
    web::Json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running at http://{}:{}",HOST.0,HOST.1);
    HttpServer::new(|| App::new()
        .wrap( Cors::default()
            .allowed_origin("localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600))
        
        .service(status)
        .service(trance_student_ex_to_js)
        .service(trance_regulate_ex_to_js)
    )
    .bind(HOST)?
    .run()
    .await
}