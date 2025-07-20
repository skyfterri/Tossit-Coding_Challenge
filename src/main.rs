use actix_web::{web, App, HttpResponse, HttpServer};
use lazy_static::lazy_static;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use actix_cors::Cors; // Import the Cors middleware

// ----- Error Handling -----
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Product not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    Internal,
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        // For better JSON error responses, let's return structured JSON
        #[derive(Serialize)]
        struct ErrorResponse {
            success: bool,
            message: String,
        }

        let (status, message) = match self {
            ApiError::Database(e) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            ApiError::NotFound => (
                actix_web::http::StatusCode::NOT_FOUND,
                "Product not found".to_string(),
            ),
            ApiError::BadRequest(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            ApiError::Internal => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        HttpResponse::build(status).json(ErrorResponse {
            success: false,
            message,
        })
    }
}

// ----- Data Models -----
#[derive(Debug, Serialize, Deserialize)]
struct Product {
    pub id: Option<i64>,
    pub name: String,
    pub price: f64,
    pub description: String,
    pub stock: i32,
}

impl Product {
    fn validate(&self) -> Result<(), ApiError> {
        if self.name.trim().is_empty() {
            return Err(ApiError::BadRequest("Product name cannot be empty".into()));
        }
        if self.price < 0.0 {
            return Err(ApiError::BadRequest("Price cannot be negative".into()));
        }
        if self.stock < 0 {
            return Err(ApiError::BadRequest("Stock cannot be negative".into()));
        }
        Ok(())
    }
}

// ----- Response Wrapper -----
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

impl<T> ApiResponse<T> {
    fn new(data: T) -> Self {
        ApiResponse { success: true, data }
    }
}

// ----- Database Connection -----
lazy_static! {
    static ref DB_CONNECTION: Arc<Mutex<Connection>> = {
        let conn = Connection::open("products.db").expect("Failed to open database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                price REAL NOT NULL,
                description TEXT NOT NULL,
                stock INTEGER NOT NULL
            )",
            [],
        )
        .expect("Failed to create products table");
        Arc::new(Mutex::new(conn))
    };
}

// ----- Handler Functions -----
async fn create_product(product: web::Json<Product>) -> Result<HttpResponse, ApiError> {
    product.validate()?; // Validate input

    let db = DB_CONNECTION.lock().map_err(|_| ApiError::Internal)?;
    db.execute(
        "INSERT INTO products (name, price, description, stock) VALUES (?1, ?2, ?3, ?4)",
        params![product.name, product.price, product.description, product.stock],
    )?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(db.last_insert_rowid())))
}

async fn get_products() -> Result<HttpResponse, ApiError> {
    let db = DB_CONNECTION.lock().map_err(|_| ApiError::Internal)?;
    let mut stmt = db.prepare("SELECT id, name, price, description, stock FROM products")?;

    let products = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                price: row.get(2)?,
                description: row.get(3)?,
                stock: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HttpResponse::Ok().json(ApiResponse::new(products)))
}

async fn get_product_by_id(id: web::Path<i64>) -> Result<HttpResponse, ApiError> {
    let db = DB_CONNECTION.lock().map_err(|_| ApiError::Internal)?;
    let product = db
        .query_row(
            "SELECT id, name, price, description, stock FROM products WHERE id = ?1",
            params![*id],
            |row| {
                Ok(Product {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    price: row.get(2)?,
                    description: row.get(3)?,
                    stock: row.get(4)?,
                })
            },
        )
        .optional()?;

    match product {
        Some(product) => Ok(HttpResponse::Ok().json(ApiResponse::new(product))),
        None => Err(ApiError::NotFound),
    }
}

async fn update_product(
    id: web::Path<i64>,
    product: web::Json<Product>,
) -> Result<HttpResponse, ApiError> {
    product.validate()?; // Validate input

    let db = DB_CONNECTION.lock().map_err(|_| ApiError::Internal)?;
    let rows_affected = db.execute(
        "UPDATE products SET name = ?1, price = ?2, description = ?3, stock = ?4 WHERE id = ?5",
        params![
            product.name,
            product.price,
            product.description,
            product.stock,
            *id
        ],
    )?;

    if rows_affected > 0 {
        Ok(HttpResponse::Ok().json(ApiResponse::new("Product updated successfully")))
    } else {
        Err(ApiError::NotFound)
    }
}

async fn delete_product(id: web::Path<i64>) -> Result<HttpResponse, ApiError> {
    let db = DB_CONNECTION.lock().map_err(|_| ApiError::Internal)?;
    let rows_affected = db.execute("DELETE FROM products WHERE id = ?1", params![*id])?;

    if rows_affected > 0 {
        Ok(HttpResponse::Ok().json(ApiResponse::new("Product deleted successfully")))
    } else {
        Err(ApiError::NotFound)
    }
}

// ----- Main Application -----
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ensure the database is initialized by accessing DB_CONNECTION
    // This will run the lazy_static block
    let _ = &*DB_CONNECTION;

    println!("Server running at http://0.0.0.0:8080/");

    HttpServer::new(|| {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin() // In a real app, specify allowed origins: .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .max_age(3600); // How long the CORS preflight request can be cached

        App::new()
            .wrap(cors) // Apply the CORS middleware
            .service(
                web::resource("/products")
                    .route(web::post().to(create_product))
                    .route(web::get().to(get_products)),
            )
            .service(
                web::resource("/products/{id}")
                    .route(web::get().to(get_product_by_id))
                    .route(web::put().to(update_product))
                    .route(web::delete().to(delete_product)),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}