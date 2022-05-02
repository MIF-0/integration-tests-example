use std::io;
use server_app::Application;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let app = Application::new("127.0.0.1:9090");
    app.run().await
}


