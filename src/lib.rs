pub mod dom;
pub mod encoded_html;
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

        // Read the HTML file
        let markup = html! {
            (DOCTYPE)
            meta charset="utf-8";
            title { (page_title) }
            h1 { (page_h1) }
            a href="spider.cloud";
            pre {
                r#"The content is ready for reading"#
            }
        }
        .into_string();

        match extractor::extract(
            &mut markup.as_bytes(),
            &url::Url::parse("https://spider.cloud").unwrap(),
            &None,
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
                println!("HTML content passed all checks.");
            }
            Err(_) => println!("error occured"),
        }
    }
}
