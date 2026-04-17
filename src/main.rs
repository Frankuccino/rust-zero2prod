use zero2prod::startup::run;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Public if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // We have removed the hard-coded "8000" - it's now coming from out settings!
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
