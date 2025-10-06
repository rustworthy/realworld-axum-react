use anyhow::Context;
use comrak::nodes::NodeHtmlBlock;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::moderation::ModerationInput;
use openai_dive::v1::resources::moderation::ModerationObject;
use openai_dive::v1::resources::moderation::ModerationParametersBuilder;
use url::Url;

pub struct Moderator {
    client: Client,
}

impl Moderator {
    pub fn new(openai_api_key: String, base_url: Option<Url>) -> Self {
        let mut client = Client::new(openai_api_key);
        if let Some(url) = base_url {
            client.set_base_url(url.as_str());
        }
        Self { client }
    }

    pub async fn moderate(&self, content: &str) -> anyhow::Result<()> {
        let (_content, image_urls) = {
            let content = content.to_owned();
            tokio::task::spawn_blocking(move || {
                let arena = comrak::Arena::new();
                let root = comrak::parse_document(&arena, &content, &comrak::Options::default());
                let mut image_urls = Vec::new();
                for node in root.descendants() {
                    match node.data.borrow().value {
                        comrak::nodes::NodeValue::Image(ref link) => {
                            if let Ok(url) = Url::parse(&link.url) {
                                image_urls.push(url);
                            } else {
                                // TODO: consider singalling back to user
                                continue;
                            };
                        }
                        comrak::nodes::NodeValue::HtmlInline(ref literal)
                        | comrak::nodes::NodeValue::HtmlBlock(NodeHtmlBlock {
                            ref literal, ..
                        }) => {
                            let fragment = scraper::Html::parse_fragment(literal);
                            for img in fragment
                                .select(&scraper::Selector::parse("img").expect("valid selector"))
                            {
                                if let Some(url) =
                                    img.value().attr("src").and_then(|src| Url::parse(src).ok())
                                {
                                    image_urls.push(url);
                                }
                            }
                        }
                        _ => continue,
                    }
                }
                (content, image_urls)
            })
            .await
            .context("failed to parse out image urls")?
        };

        let objects = image_urls
            .iter()
            .map(|url| ModerationObject::image_url(url.as_str()));
        // TODO: push content as text node
        for object in objects {
            dbg!(&object);
            let parameters = ModerationParametersBuilder::default()
                .model("omni-moderation-latest")
                .input(ModerationInput::MultiModal(vec![object]))
                .build()
                .context("failed to build moderation parameters")?;

            match self.client.moderations().create(parameters).await {
                Err(e) => {
                    eprintln!("{e:#}");
                }
                Ok(res) => {
                    println!("{res:#?}");
                }
            }
        }
        Ok(())
    }
}
