use std::str::FromStr;

use neon::prelude::*;
use spinners_rs::{Spinner, Spinners};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn create_spinner(mut cx: FunctionContext) -> JsResult<JsObject> {
    let spinner_name = cx.argument::<JsString>(0)?.value(&mut cx);
    let message = cx.argument::<JsString>(1)?.value(&mut cx);

    let spinner_type = match Spinners::from_str(&spinner_name) {
        Ok(v) => v,
        Err(_) => return cx.throw_error("invalid spinner name"),
    };

    let mut spinner = Spinner::new(spinner_type, message);
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("createSpinner", create_spinner)?;
    Ok(())
}
