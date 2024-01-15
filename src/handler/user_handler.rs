use salvo::handler;

#[handler]
pub async fn login() -> &'static str {
    "login"
}