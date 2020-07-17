use async_std::sync::Arc;
use handlebars::Handlebars;
use std::collections::BTreeMap;
use tide_handlebars::prelude::*;

#[derive(Clone)]
pub struct HandlebarsEngine {
    registry: Arc<Handlebars<'static>>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();
    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "./examples/templates/")
        .unwrap();
    let engine = HandlebarsEngine {
        registry: Arc::new(hb),
    };

    let mut app = tide::with_state(engine);

    app.at("/")
        .get(|req: tide::Request<HandlebarsEngine>| async move {
            let hb = &req.state().registry;
            let mut data0 = BTreeMap::new();
            data0.insert("title".to_string(), "hello tide!".to_string());
            data0.insert("parent".to_string(), "base".to_string());
            Ok(hb.render_response_ext("content", &data0, "html")?)
        });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
