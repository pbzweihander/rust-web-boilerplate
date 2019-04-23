use {
    futures::executor::block_on,
    http_service::Body,
    http_service_mock::make_server,
    rust_web_boilerplate::{config::Opt, make_app},
    tide::http::Request,
};

#[test]
fn integration_test() {
    let opt = Opt {
        host: "0.0.0.0".to_string(),
        port: 5000,
    };
    let app = make_app(opt);

    let mut server = make_server(app.into_http_service()).unwrap();

    let req = Request::get("/foobar").body(Body::empty()).unwrap();
    let resp = server.simulate(req).unwrap();
    assert_eq!(resp.status(), 404);

    let req = Request::get("/ping").body(Body::empty()).unwrap();
    let resp = server.simulate(req).unwrap();
    assert_eq!(resp.status(), 200);
    let body = block_on(resp.into_body().into_vec()).unwrap();
    let body_str = String::from_utf8(body).unwrap();
    assert_eq!(&body_str, "OK");

    let req = Request::get("/hello/foobar").body(Body::empty()).unwrap();
    let resp = server.simulate(req).unwrap();
    assert_eq!(resp.status(), 200);
    let body = block_on(resp.into_body().into_vec()).unwrap();
    let body_str = String::from_utf8(body).unwrap();
    assert_eq!(&body_str, "Hello, foobar!");

    let req = Request::post("/hello/foobar").body("baz".into()).unwrap();
    let resp = server.simulate(req).unwrap();
    assert_eq!(resp.status(), 200);
    let body = block_on(resp.into_body().into_vec()).unwrap();
    let body_str = String::from_utf8(body).unwrap();
    assert_eq!(&body_str, "Hello, foobar!\nYour message was \"baz\".");
}
