use handlebars::Handlebars;
use std::collections::BTreeMap;
use tide_handlebars::prelude::*;

struct HandlebarsEngine {
    registry: Handlebars<'static>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut engine = HandlebarsEngine {
        registry: Handlebars::new(),
    };

    engine
        .registry
        .register_template_file("simple.html", "./examples/templates/simple.html")
        .unwrap();

    let mut app = tide::with_state(engine);
    app.at("/:name")
        .get(|req: tide::Request<HandlebarsEngine>| async move {
            let hb = &req.state().registry;
            let name: String = req.param("name")?;
            let mut data0 = BTreeMap::new();
            data0.insert("name".to_string(), name);
            Ok(hb.render_response("simple.html", &data0)?)
        });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
