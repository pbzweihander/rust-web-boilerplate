mod api;
pub mod config;

use {crate::config::Config, tide::Server};

pub fn make_server(config: Config) -> Server<Config> {
    let mut app = tide::with_state(config);
    app.at("/ping").get(|_| async { Ok("OK") });
    app.at("/hello/:name")
        .get(api::hello)
        .post(api::hello_with_body);

    app
}
