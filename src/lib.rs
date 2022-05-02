use std::io;
use actix_web::{App, HttpResponse, HttpServer, Responder};

pub struct Application {
    address: String,
}

impl Application {
    pub fn new(address: &str) -> Application {
        let address = String::from(address);
        Application {
            address
        }
    }

    pub async fn run(&self) -> io::Result<()> {
        dotenv::from_filename(".env")
            .or_else(|_| dotenv::from_filename("../.env"))
            .expect(".env file not found");
        env_logger::init();

        HttpServer::new(move || App::new()
            .service(hello)
        )
            .bind(self.address.as_str())?
            .run()
            .await
    }
}

#[actix_web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}