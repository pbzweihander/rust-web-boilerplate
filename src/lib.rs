#![feature(async_await)]

mod api;
pub mod config;

use {crate::config::Opt, tide::App};

pub fn make_app(opt: Opt) -> App<Opt> {
    let mut app = tide::App::new(opt);
    app.middleware(tide::middleware::RootLogger::new());
    app.at("/ping").get(async move |_| "OK");
    app.at("/hello/:name")
        .get(api::hello)
        .post(api::hello_with_body);

    app
}
