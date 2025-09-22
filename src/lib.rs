use worker::*;

/// Exceptions
///
/// see: https://github.com/beancount/fava/blob/main/src/fava/helpers.py
#[derive(Debug, PartialEq)]
enum Helpers {
    /// TODO: Option<Directive>, Option<Source>
    BeancountError(String),
    FavaError(String)
}

mod beans;
mod core;

#[event(fetch)]
async fn fetch(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();
    Response::ok("Hello World!")
}