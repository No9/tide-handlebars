use tide_handlebars::prelude::*;
use handlebars::Handlebars;
use std::collections::BTreeMap;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("simple.html", "./templates/simple.html")
        .unwrap();
    let mut app = tide::with_state(handlebars);
    app.listen("127.0.0.1:8080").await?;

    app.at("/:name")
        .get(|req: tide::Request<Handlebars>| async move {
            let hb = req.state();
            let name: String = req.param("name")?;
            let mut data0 = BTreeMap::new();
            data0.insert("name".to_string(), name);
            hb.render_response("hello.html", &data0)
        });


    Ok(())
}
