use handlebars::Handlebars;
use serde::Serialize;
use std::path::PathBuf;
use tide::{http::Mime, Body, Response, Result};

pub trait TideHandlebarsExt {
    fn render_response<T>(&self, template_name: &str, context: &T) -> Result
    where
        T: Serialize;
    fn render_body<T>(&self, template_name: &str, context: &T) -> Result<Body>
    where
        T: Serialize;
}


impl TideHandlebarsExt for Handlebars <'_>{
    fn render_body<T>(&self, template_name: &str, context: &T) -> Result<Body>
    where
        T: Serialize,
    {
        let string = self.render(template_name, context)?;
        let mut body = Body::from_string(string);

        let path = PathBuf::from(template_name);
        if let Some(extension) = path.extension() {
            if let Some(mime) = Mime::from_extension(extension.to_string_lossy()) {
                body.set_mime(mime)
            }
        }

        Ok(body)
    }

    fn render_response<T>(&self, template_name: &str, context: &T) -> Result
    where
        T: Serialize,
    {
        let mut response = Response::new(200);
        response.set_body(self.render_body(template_name, context)?);
        Ok(response)
    }
}

#[macro_export]
macro_rules! registry {
    ($($key:expr => $value:expr,)+) => { registry!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let mut _registry = ::Handlebars::new();
            $(
                _registry.register_templates_directory($key, &$value);
            )*
            _registry
        }
     };
}

pub mod prelude {
    pub use super::{registry, TideHandlebarsExt};
}

#[cfg(test)]
mod tests {

    use super::*;
    use async_std::prelude::*;
    use std::collections::BTreeMap;

    #[async_std::test]
    async fn test_body() {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_template_file("simple.html", "./tests/templates/simple.html")
            .unwrap();

        let mut data0 = BTreeMap::new();
        data0.insert("title".to_string(), "hello tide!".to_string());
        let mut body = handlebars.render_body("simple.html", &data0).unwrap();

        assert_eq!(body.mime(), &tide::http::mime::HTML);

        let mut body_string = String::new();
        body.read_to_string(&mut body_string).await.unwrap();
        assert_eq!(body_string, "<h1>hello tide!</h1>\n");
    }

    #[async_std::test]
    async fn response() {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("simple.html", "./tests/templates/simple.html")
            .unwrap();
        let mut data0 = BTreeMap::new();
        data0.insert("title".to_string(), "hello tide!".to_string());

        let mut response = handlebars.render_response("simple.html", &data0).unwrap();

        assert_eq!(response.content_type(), Some(tide::http::mime::HTML));

        let http_response: &mut tide::http::Response = response.as_mut();
        let body_string = http_response.body_string().await.unwrap();
        assert_eq!(body_string, "<h1>hello tide!</h1>\n");
    }

    #[test]
    fn unknown_content_type() {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./tests/templates")
            .unwrap();

        let mut data0 = BTreeMap::new();
        data0.insert("title".to_string(), "hello tide!".to_string());
        let body = handlebars.render_body("simple", &data0).unwrap();

        assert_eq!(body.mime(), &tide::http::mime::PLAIN);
    }

    // Templates are validate on load in handlebars -- need to work into the component
    // #[test]
    // fn bad_template() {
    //     let mut handlebars = Handlebars::new();
    //     handlebars
    //         .register_templates_directory(".broken", "./tests/templates")
    //         .unwrap();

    //     let mut data0 = BTreeMap::new();
    //     data0.insert("title".to_string(), "hello tide!".to_string());
    //     let result = handlebars.render_body("simple", &data0);

    //     assert!(result.is_err());
    // }
}
