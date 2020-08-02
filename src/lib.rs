//! # Tide-Handlebars integration This crate exposes [an extension
//! trait](TideHandlebarsExt) that adds two methods to [`handlebars::Handlebars`]:
//! [`render_response`](TideHandlebarsExt::render_response) and
//! [`render_body`](TideHandlebarsExt::render_body).
//! [`Handlebars`](handlebars::Handlebars)s.
use handlebars::Handlebars;
use serde::Serialize;
use std::path::PathBuf;
use tide::{http::Mime, Body, Response, Result};

/// This extension trait adds two methods to [`handlebars::Handlebars`]:
/// [`render_response`](TideHandlebarsExt::render_response) and
/// [`render_body`](TideHandlebarsExt::render_body)
pub trait TideHandlebarsExt {
    /// `render_body` returns a fully-rendered [`tide::Body`] with mime
    /// type set based on the template name file extension using the
    /// logic at [`tide::http::Mime::from_extension`]. This will
    /// return an `Err` variant if the render was unsuccessful.
    ///
    /// ```rust
    /// use handlebars::Handlebars;
    /// use tide_handlebars::prelude::*;
    /// use std::collections::BTreeMap;
    /// let mut handlebars = Handlebars::new();
    ///     handlebars
    ///     .register_template_file("simple.html", "./tests/templates/simple.html")
    ///     .unwrap();
    ///
    /// let mut data0 = BTreeMap::new();
    /// data0.insert("title".to_string(), "hello tide!".to_string());
    /// let mut body = handlebars.render_body("simple.html", &data0).unwrap();
    /// assert_eq!(body.mime(), &tide::http::mime::HTML);
    ///```
    fn render_body<T>(&self, template_name: &str, context: &T) -> Result<Body>
    where
        T: Serialize;
    /// `render_body_ext` returns a fully-rendered [`tide::Body`] with mime
    /// type set based on the extension using the
    /// logic at [`tide::http::Mime::from_extension`]. This will
    /// return an `Err` variant if the render was unsuccessful.
    ///
    /// ```rust
    /// use handlebars::Handlebars;
    /// use tide_handlebars::prelude::*;
    /// use std::collections::BTreeMap;
    /// let mut handlebars = Handlebars::new();
    ///     handlebars
    ///     .register_template_file("simple.hbs", "./tests/templates/simple.hbs")
    ///     .unwrap();
    ///
    /// let mut data0 = BTreeMap::new();
    /// data0.insert("title".to_string(), "hello tide!".to_string());
    /// let mut body = handlebars.render_body_ext("simple.hbs", &data0, "html").unwrap();
    /// assert_eq!(body.mime(), &tide::http::mime::HTML);
    ///```
    fn render_body_ext<T>(&self, template_name: &str, context: &T, extension: &str) -> Result<Body>
    where
        T: Serialize;
    /// `render_response` returns a tide Response with a body rendered
    /// with [`render_body`](TideHandlebarsExt::render_body). This will
    /// return an `Err` variant if the render was unsuccessful.
    ///
    /// ```rust
    /// use handlebars::Handlebars;
    /// use tide_handlebars::prelude::*;
    /// use std::collections::BTreeMap;
    /// let mut handlebars = Handlebars::new();
    /// handlebars
    ///     .register_template_file("simple.html", "./tests/templates/simple.html")
    ///     .unwrap();
    /// let mut data0 = BTreeMap::new();
    /// data0.insert("title".to_string(), "hello tide!".to_string());
    /// let mut response = handlebars.render_response("simple.html", &data0).unwrap();
    /// assert_eq!(response.content_type(), Some(tide::http::mime::HTML));
    ///```
    fn render_response<T>(&self, template_name: &str, context: &T) -> Result
    where
        T: Serialize;
    /// `render_response_ext` returns a tide Response with a body rendered
    /// with [`render_body`](TideHandlebarsExt::render_body). This will
    /// return an `Err` variant if the render was unsuccessful.
    ///
    /// ```rust
    /// use handlebars::Handlebars;
    /// use tide_handlebars::prelude::*;
    /// use std::collections::BTreeMap;
    /// let mut handlebars = Handlebars::new();
    /// handlebars
    ///     .register_template_file("simple.hbs", "./tests/templates/simple.hbs")
    ///     .unwrap();
    /// let mut data0 = BTreeMap::new();
    /// data0.insert("title".to_string(), "hello tide!".to_string());
    /// let mut response = handlebars.render_response_ext("simple.hbs", &data0, "html").unwrap();
    /// assert_eq!(response.content_type(), Some(tide::http::mime::HTML));
    ///```
    fn render_response_ext<T>(&self, template_name: &str, context: &T, extension: &str) -> Result
    where
        T: Serialize;
}

impl TideHandlebarsExt for Handlebars<'_> {
    fn render_body_ext<T>(&self, template_name: &str, context: &T, extension: &str) -> Result<Body>
    where
        T: Serialize,
    {
        let string = self.render(template_name, context)?;
        let mut body = Body::from_string(string);
        if let Some(mime) = Mime::from_extension(extension) {
            body.set_mime(mime);
        }
        Ok(body)
    }
    fn render_body<T>(&self, template_name: &str, context: &T) -> Result<Body>
    where
        T: Serialize,
    {
        let string = self.render(template_name, context)?;

        let path = PathBuf::from(template_name);
        let mut body = Body::from_string(string);
        if let Some(extension) = path.extension() {
            if let Some(mime) = Mime::from_extension(extension.to_string_lossy()) {
                body.set_mime(mime);
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
    fn render_response_ext<T>(&self, template_name: &str, context: &T, extension: &str) -> Result
    where
        T: Serialize,
    {
        let mut response = Response::new(200);
        response.set_body(self.render_body_ext(template_name, context, extension)?);
        Ok(response)
    }
}

pub mod prelude {
    pub use super::TideHandlebarsExt;
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
    #[test]
    fn body_with_extension() {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./tests/templates")
            .unwrap();

        let mut data0 = BTreeMap::new();
        data0.insert("title".to_string(), "hello tide!".to_string());
        let body = handlebars
            .render_body_ext("simple", &data0, "html")
            .unwrap();

        assert_eq!(body.mime(), &tide::http::mime::HTML);
    }
    #[async_std::test]
    async fn response_with_extension() {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./tests/templates")
            .unwrap();
        let mut data0 = BTreeMap::new();
        data0.insert("title".to_string(), "hello tide!".to_string());

        let mut response = handlebars
            .render_response_ext("simple", &data0, "html")
            .unwrap();

        assert_eq!(response.content_type(), Some(tide::http::mime::HTML));

        let http_response: &mut tide::http::Response = response.as_mut();
        let body_string = http_response.body_string().await.unwrap();
        assert_eq!(body_string, "<h1>hello tide!</h1>\n");
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
