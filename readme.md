### Crud API RUST

This is a simple CRUD API using Rust and Actix-web.

### How to run
````
git clone
cd rust-crud-api
cargo run
````

### Endpoints
```
GET /product
GET /product/{id}
POST /product
PUT /product/{id}
DELETE /product/{id}
```

### Payload
```
{
    "name": "Product Name",
    "price": 100.00
}
```