use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}
#[derive(Serialize, Deserialize)]
struct Article {
    name: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    let raw_json: &str = r#"
    {
        "name": "My Article",
        "author": "John Doe",
        "paragraphs": [
            {
                "name": "Introduction"
            },
            {
                "name": "Body"
            },
            {
                "name": "Conclusion"
            }
        ]
    }
    "#;

    let article: Article = serde_json::from_str(raw_json).unwrap();

    println!("Name: {}", article.name);
}
