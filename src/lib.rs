pub mod dom;
pub mod error;
pub mod extractor;
pub mod scorer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_readability() {
        use maud::{html, DOCTYPE};

        let page_title = "Readability Test";
        let page_h1 = "Reading is fun";

        let markup = html! {
            (DOCTYPE)
            html lang="fr" {
                meta charset="utf-8";
                title { (page_title) }
                h1 { (page_h1) }
                a href="spider.cloud";
                pre {
                    r#"The content is ready for reading"#
                }
            }
        }
        .into_string();

        match extractor::extract(
            &mut markup.as_bytes(),
            &url::Url::parse("https://spider.cloud").unwrap(),
        ) {
            Ok(product) => {
                assert!(
                    product
                        .content
                        .contains(&format!("<title>{}</title>", page_title)),
                    "Title is missing or incorrect"
                );
                assert!(
                    product.content.contains(&format!("<h1>{page_h1}</h1>")),
                    "H1 tag is missing or incorrect"
                );
                assert!(
                    product.content.contains("The content is ready for reading"),
                    "Expected phrase is missing"
                );
                assert!(
                    product
                        .content
                        .contains(&r###"<html class="paper" lang="fr">"###),
                    "Html lang is missing or incorrect"
                );
            }
            Err(_) => println!("error occured"),
        }
    }
}
