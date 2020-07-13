use {
    rust_web_boilerplate::{config::Config, make_server},
    tide::http::{Body, Request, Response, Url},
};

#[async_std::test]
async fn test_not_found() {
    let server = make_server(Config {
        host: "0.0.0.0".to_string(),
        port: 5000,
    });

    let mut req = Request::get(Url::parse("http://localhost:5000/foobar").unwrap());
    req.set_body(Body::empty());
    let resp: Response = server.respond(req).await.unwrap();
    assert_eq!(resp.status(), 404);
}

#[async_std::test]
async fn test_ping() {
    let server = make_server(Config {
        host: "0.0.0.0".to_string(),
        port: 5000,
    });

    let mut req = Request::get(Url::parse("http://localhost:5000/ping").unwrap());
    req.set_body(Body::empty());
    let mut resp: Response = server.respond(req).await.unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.body_string().await.unwrap();
    assert_eq!(&body, "OK");
}

#[async_std::test]
async fn test_hello() {
    let server = make_server(Config {
        host: "0.0.0.0".to_string(),
        port: 5000,
    });

    let mut req = Request::get(Url::parse("http://localhost:5000/hello/foobar").unwrap());
    req.set_body(Body::empty());
    let mut resp: Response = server.respond(req).await.unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.body_string().await.unwrap();
    assert_eq!(&body, "Hello, foobar!");
}

#[async_std::test]
async fn test_hello_with_body() {
    let server = make_server(Config {
        host: "0.0.0.0".to_string(),
        port: 5000,
    });

    let mut req = Request::post(Url::parse("http://localhost:5000/hello/foobar").unwrap());
    req.set_body("baz");
    let mut resp: Response = server.respond(req).await.unwrap();
    assert_eq!(resp.status(), 200);
    let body = resp.body_string().await.unwrap();
    assert_eq!(&body, "Hello, foobar!\nYour message was \"baz\".");
}
