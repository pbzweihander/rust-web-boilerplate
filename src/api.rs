use {
    crate::config::Opt,
    tide::{
        error::{EndpointResult, ResultExt},
        response::IntoResponse,
        Context, Response,
    },
};

pub async fn hello(ctx: Context<Opt>) -> EndpointResult<Response> {
    let name: String = ctx.param("name").client_err()?;

    Ok(format!("Hello, {}!", name).into_response())
}

pub async fn hello_with_body(mut ctx: Context<Opt>) -> EndpointResult<Response> {
    let name: String = ctx.param("name").client_err()?;
    let msg = await!(ctx.body_string()).client_err()?;

    Ok(format!("Hello, {}!\nYour message was \"{}\".", name, msg).into_response())
}
