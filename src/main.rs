use rust_web_boilerplate::{config::Config, make_server};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let config = Config::from_args();
    let (host, port) = (config.host.clone(), config.port);

    let server = make_server(config);

    server.listen((host.as_ref(), port)).await?;

    Ok(())
}
