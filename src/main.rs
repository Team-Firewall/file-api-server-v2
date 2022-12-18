
use actix_easy_multipart::MultipartForm;
use actix_easy_multipart::tempfile::Tempfile;
use actix_web::{get, post,web, App, HttpResponse, HttpServer, Responder};
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

#[derive(MultipartForm)]
struct Form {
    file_set: Option<Tempfile>,
}

#[get("/")]
async fn status() -> impl Responder {
    HttpResponse::Ok().body("200")
}

#[post("/trance")]
async fn trance_ex_to_js(req_excel: MultipartForm<Form>) -> web::Json<Vec<UserData>> {

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running at http://{}:{}",HOST.0,HOST.1);
    HttpServer::new(|| {
        App::new()
            .service(status)
            .service(trance_ex_to_js)
    })
    .bind(HOST)?
    .run()
    .await
}