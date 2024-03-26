use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct ProductList {
    products: Mutex<Vec<Product>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Product {
    id: u32,
    name: String,
    price: u32,
}

async fn index(data: web::Data<ProductList>) -> impl Responder {
    match data.products.lock() {
        Ok(products) => HttpResponse::Ok().json(products.to_vec()),
        Err(_) => HttpResponse::BadRequest().finish()
    }
}

async fn create(req: web::Json<Product>, data: web::Data<ProductList>) -> impl Responder {
    let product = Product {
        id: req.id,
        name: req.name.clone(),
        price: req.price,
    };

    match data.products.lock() {
        Ok(mut produc_list) => {
            produc_list.push(product);
            HttpResponse::NoContent().finish()
        },
        Err(_) => {
            HttpResponse::BadRequest().finish()
        }
    }
}

async fn update(req: web::Json<Product>, param: web::Path<u32>, data: web::Data<ProductList>) -> impl Responder {
    let id = param.into_inner();
    let product = Product {
        id: req.id,
        name: req.name.clone(),
        price: req.price,
    };

    match data.products.lock() {
        Ok(mut products) => {
            for i in 0..products.len() {
                if id == products[i].id {
                    products[i].id = product.id;
                    products[i].name = product.name.clone();
                    products[i].price = product.price;
                }
            }
            HttpResponse::Ok().json(product)
        },
        Err(_) => HttpResponse::BadRequest().finish()
    }
}

async fn delete(param: web::Path<u32>, data: web::Data<ProductList>) -> impl Responder {
    let id = param.into_inner();
    match data.products.lock() {
        Ok(mut products) => {
            *products = products.to_vec().into_iter().filter(|product| product.id != id).collect();
            HttpResponse::Ok().json(products.to_vec())
        },
        Err(_) => HttpResponse::BadRequest().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(ProductList {
        products: Mutex::new(vec![]),
    });
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(app_data.clone())
            .route("/product", web::get().to(index))
            .route("/product", web::post().to(create))
            .route("/product/{id}", web::put().to(update))
            .route("/product/{id}", web::delete().to(delete))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
