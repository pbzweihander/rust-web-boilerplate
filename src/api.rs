use {
    crate::config::Config,
    tide::{Request, Result},
};

pub async fn hello(req: Request<Config>) -> Result {
    let name: String = req.param("name")?;

    Ok(format!("Hello, {}!", name).into())
}

pub async fn hello_with_body(mut req: Request<Config>) -> Result {
    let name: String = req.param("name")?;
    let msg = req.body_string().await?;

    Ok(format!("Hello, {}!\nYour message was \"{}\".", name, msg).into())
}
