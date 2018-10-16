use db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Book, NewBook};
use serde_json::Value;

#[get("/books", format = "application/json")]
pub fn index(conn: DbConn) -> Json<Value> {
    let books = Book::all(&conn);

    Json(json!({
        "status": 200,
        "result": books,
    }))
}

#[post("/books", format = "application/json", data = "<new_book>")]
pub fn new(conn: DbConn, new_book: Json<NewBook>) -> Json<Value> {
    Json(json!({
        "status": Book::insert(new_book.into_inner(), &conn),
        "result": Book::all(&conn).first(),
    }))
}

#[get("/books/<id>", format = "application/json")]
pub fn show(conn: DbConn, id: i32) -> Json<Value> {
    let result = Book::show(id, &conn);
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status, 
        "result": result.get(0),
    }))
}

#[put("/books/<id>", format = "application/json", data = "<book>")]
pub fn update(conn: DbConn, id: i32, book: Json<NewBook>) -> Json<Value> {
    let status = if Book::update_by_id(id, &conn, book.into_inner()) {
        200
    } else {
        404
    };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/books/<id>")]
pub fn delete(id: i32, conn: DbConn) -> Json<Value> {
    let status = if Book::delete_by_id(id, &conn) {
        200
    } else {
        404
    };
    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/books/authors/<author>", format = "application/json")]
pub fn author(author: String, conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Book::all_by_author(author, &conn),
    }))
}


//#[catch(404)]
//fn not_found() -> Json<Value> {
//    Json(json!({
//        "status": "error",
//        "reason": "Resource was not found"
//    }))
//}
//needs to be reworked for current version of rocket.
