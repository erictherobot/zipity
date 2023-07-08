// Example: http://localhost:8080/api/users

use serde_json::json;

pub async fn users() -> impl Responder {
    let users = vec![
        json!({"id": 1, "name": "John"}),
        json!({"id": 2, "name": "Jane"}),
        json!({"id": 3, "name": "Alice"})
    ];

    HttpResponse::Ok().json(users)
}
