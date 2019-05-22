use rust_web_boilerplate::{config::Opt, make_app};

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let opt = Opt::from_args();
    let (host, port) = (opt.host.clone(), opt.port);

    let app = make_app(opt);

    app.serve((host.as_ref(), port)).unwrap();
}
