// // use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

// // fn main() {
// //     println!("Hello, world!");
// //     //Bind My server youseing Local ip adress

// //     let listener = TcpListener::bind("127.0.01:8082").expect("Port bind Faikled");
// //     println!("Server. Running at http://127.0.01:8082");

// //     for stream in listener.incoming(){
// //         let stream = stream.expect("Failed to accept connection");
// //         handle_connection(stream);
// //     }
// // }

// // fn handle_connection(mut stream : TcpStream){
// //     let mut buffer = [0 ; 1024];
// //     let _ = stream.read(&mut buffer).unwrap_or(0);
// //     let request = String::from_utf8_lossy(&buffer[..]);
// //     let (status_line , content) = if request.starts_with("GET / ") {
// //         (
// //             "HTTP/1.1 200 OK\r\nContent-Type : text/html\r\n\r\n",
// //             "<h1>Well come to my rust serve</h1>",
// //         )
// //     }else if request.starts_with("GET /hello/"){
// //         (
// //             "HTTP/1.1 200 OK\r\nContent-Type : text/html\r\n\r\n",
// //             "<h1>Hello From Rust</h1>"
// //         )
// //     }else{
// //         (
// //             "HTTP/1.1 400 NOT FOUND\r\nContent-Type : text/html\r\n\r\n",
// //             "<h1>Page Not Found</h1>"
// //         )
// //     };
// //     let response = format!("{}{}" , status_line , content);
// //     let _  = stream.write_all(response.as_bytes());
// //     let _ = stream.flush();
// // }

// // use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

// // fn main(){
// //   println!("This is a manojseetaram");
// //   let listener = TcpListener::bind("127.0.0.1:8084").expect("Failed to server connection");
// //     println!("The server running on http://127.0.0.1:8084");

// //       for stream in listener.incoming(){
// //         let stream = stream.expect("Faield to accept connection");
// //         std::thread::spawn(|| {
// //          handle_connection(stream);
// //         });
// //       }


// //       fn handle_connection(mut stream  : TcpStream){
// //        let mut buffer = [0;1024];
// //        let _stream = stream.read(&mut buffer).unwrap_or(0);
// //        //convert to the byte in to human readbele string
// //        let request = String::from_utf8_lossy(&buffer[..]);
// //        let (status_line , content) = if request.starts_with("GET / "){
// //         (
// //           "HTTP/1.1 200 OK \r\nContent-Type : text/html\r\n\r\n",
// //           "<h1>Well done this is Home page</h1>"
// //         )
// //        }else if request.starts_with("GET /hello"){
// //         (
// //           "HTTP/1/1 200 OK \r\nContent-Type : text/html\r\n\r\n",
// //           "<h1>This is a Hello page</h1>"
// //         )
// //        }else{
// //         (
// //           "HTTP/1/1 400 NOT FOUND \r\nContent-Type : text/html\r\n\r\n",
// //           "<h1>Page is not found</h1>"
// //         )
// //        };
// //        let response = format!("{} {}" , content , status_line);
// //        let _ = stream.write_all(response.as_bytes());
// //        let _ = stream.flush();

// //       }
// // }

// use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

// fn main(){
//   println!("This is a multi threding server");
//   let listener = TcpListener::bind("127.0.0.1:8080").expect("fAILED TO pORT BIND");
//   println!("Server running on http://127.0.0.1:8080");

//   for stream in listener.incoming(){
//     let stream = stream.expect("Failed to accept connecion");
//     std::thread::spawn(||{
//   handle_connection(stream)
//     });
//   };
//   fn handle_connection(mut stream : TcpStream){
//     let mut buffer = [0;1024];
//     let _stream = stream.read(&mut buffer).unwrap_or(0);
//      //Concert to byte in to human readble string;
//      let request = String::from_utf8_lossy(&buffer [..]);
//      let (status_line , content) = if request.starts_with("GET /"){
// (
//       "HTTP/1/1 200 OK \r\nContent-Type :  html/text/r/n/r/n",
//       "<h1>This is a maon</h1>"

// )
//      }else if request.starts_with("GET /hello"){
// (
//    "HTTP/1/1 200 OK \r\nContent-Type :  html/text/r/n/r/n",
//       "<h1>This is a maon</h1>"
// )
//      }else{
//       (
//     "HTTP/1/1 400 NOT FOUND \r\nContent-Type : html/text/r/n/r/n",
//     "<h1>page not found</h1>"
//       )
//      };


use std::env;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use bson::{doc, Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client};
use dotenv::dotenv;

// import your models including Post
use crate::models::{AppError, AppState, CreatePost, UpdatePost, Post};

mod db;
mod models;

async fn get_post(data: web::Data<AppState>, path: web::Path<String>) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::BadRequest("Invalid ObjectId".into()))?;
    let filter = doc! { "_id": oid };

    // find_one takes only the filter (mongodb 3.2.4)
    match data.collection.find_one(filter).await.map_err(AppError::Db)? {
        Some(doc) => {
            // explicitly tell the compiler we want a Post
            let post: Post = bson::from_document(doc).map_err(|e| AppError::Internal(e.to_string()))?;
            Ok(HttpResponse::Ok().json(post))
        }
        None => Err(AppError::NotFound("Post not found".into())),
    }
}

async fn create_post(data: web::Data<AppState>, payload: web::Json<CreatePost>) -> Result<HttpResponse, AppError> {
    let doc = doc! { "title": &payload.title, "content": &payload.content };

    // insert_one takes only one arg (the document)
    let insert_result = data.collection.insert_one(doc).await.map_err(AppError::Db)?;

    // robust extraction of ObjectId from Bson
    let oid = match insert_result.inserted_id {
        Bson::ObjectId(oid) => oid,
        _ => return Err(AppError::Internal("Failed to read inserted id".into())),
    };

    let post = Post {
        id: Some(oid),
        title: payload.title.clone(),
        content: payload.content.clone(),
    };
    Ok(HttpResponse::Created().json(post))
}

async fn update_post(
    data: web::Data<AppState>,
    path: web::Path<String>,
    payload: web::Json<UpdatePost>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::BadRequest("Invalid object id".into()))?;

    let mut update_doc = Document::new();
    if let Some(title) = &payload.title {
        update_doc.insert("title", title.clone());
    }
    if let Some(content) = &payload.content {
        update_doc.insert("content", content.clone());
    }

    if update_doc.is_empty() {
        return Err(AppError::BadRequest("No fields provided for update".into()));
    }

    let filter = doc! { "_id": oid.clone() };
    let update = doc! { "$set": update_doc };

    // update_one(filter, update)
    let result = data.collection.update_one(filter.clone(), update).await.map_err(AppError::Db)?;
    if result.matched_count == 0 {
        return Err(AppError::NotFound("Post not found".into()));
    }

    let doc = data.collection.find_one(filter).await.map_err(AppError::Db)?
        .ok_or(AppError::Internal("Failed to fetch updated document".into()))?;

    // annotate target type explicitly
    let post: Post = bson::from_document(doc).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(HttpResponse::Ok().json(post))
}

async fn delete_post(data: web::Data<AppState>, path: web::Path<String>) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let oid = ObjectId::parse_str(&id).map_err(|_| AppError::BadRequest("Invalid object id ".into()))?;
    let filter = doc! {"_id": oid};
    let result = data.collection.delete_one(filter).await.map_err(AppError::Db)?;
    if result.deleted_count == 1 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(AppError::NotFound("Post not found".into()))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // ✅ fixed dotenv import
    env_logger::init();

    let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017/todo".into());
    let mut client_options =
        ClientOptions::parse(&mongo_uri).await.expect("Failed parse mongo options");

    client_options.app_name = Some("rust_mongo-api".to_string());
    let client = Client::with_options(client_options).expect("Failed to initialize mongo db client");

    let db = client.database("rust_api");
    let collection = db.collection::<Document>("posts"); // ✅ ensure typed collection
    let shared_data = web::Data::new(AppState { collection });

    println!("Server running on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(shared_data.clone()) 127 0.01
            .route("/posts", web::post().to(create_post))
            .route("/posts/{id}", web::get().to(get_post))
            .route("/posts/{id}", web::put().to(update_post))
            .route("/posts/{id}", web::delete().to(delete_post))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
