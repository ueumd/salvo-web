use salvo::prelude::*;

#[handler]
pub async fn hello() -> &'static str {
    "Hello Salvo"
}