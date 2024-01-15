mod config;
mod handler;

use once_cell::sync::Lazy;
use rbatis::RBatis;
use salvo::prelude::*;

use crate::handler::hello_handler::{*};
use crate::handler::user_handler::{*};

pub static RB: Lazy<RBatis> = Lazy::new(RBatis::new);

#[tokio::main]
async fn main() {

    log4rs::init_file("src/log4rs.yaml", Default::default()).unwrap();

    let config_info = config::init_load_config::load_global_config().unwrap();

    // let mysql_url = "mysql://root:Abcdef@123456@127.0.0.1:3306/salvo_web";
    let mysql_url =  format!("mysql:://{}:{}@{}:{}/{}",
                             config_info.mysql.username,
                             config_info.mysql.password,
                             config_info.mysql.host,
                             config_info.mysql.port,
                             config_info.mysql.db_name,
    );
    RB.init(rbdc_mysql::driver::MysqlDriver {}, &mysql_url).unwrap();

    let serve_url = format!("{}:{}",config_info.application.server.addr, config_info.application.server.port);
    let acceptor = TcpListener::new(serve_url.to_string()).bind().await;

    println!("http server started listening on http://127.0.0.1:{}", config_info.application.server.port.to_string());

    Server::new(acceptor).serve(route()).await;
}


fn route() -> Router {
    Router::new().get(hello)
        .push(Router::new().path("/api")
        .push(Router::new().path("login").get(login)))
}