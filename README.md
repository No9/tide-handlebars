# tide-handlebars

This crate exposes an extension trait that adds four functions to handlebars::Handlebars: 

* `render_response` - Render the template and return a tide response using the template name to assume the content type
    e.g. `template.html` would set the content type to be `text/html`

* `render_body`- Render the template and return a tide body using the template name to assume the content type
    e.g. `template` defaults the content type to be `text/plain`

* `render_response_ext` - Render the template and return a tide response specifying the file extension explicitly
    e.g. `"html"` would set the content type to be `text/html`

* `render_body_ext` - Render the template and return a tide body using the template extension to assume the content type


## Documentation 

* [Rust Documentation](https://docs.rs/tide-handlebars)

* [Examples](https://github.com/No9/tide-handlebars/blob/master/examples/)

---

<a href="https://crates.io/crates/tide-handlebars">
<img src="https://img.shields.io/crates/v/tide-handlebars.svg?style=flat-square"
alt="Crates.io version" />
</a>

<a href="https://docs.rs/tide-handlebars">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
alt="docs.rs docs" />
</a>

---

## usage

```rust
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
```

See the [main handlebars repo](https://github.com/sunng87/handlebars-rust) for full details on handlebars usage.
