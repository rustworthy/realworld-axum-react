use anyhow::Context;
use openai_dive::v1::api::Client;
use openai_dive::v1::resources::moderation::ModerationInput;
use openai_dive::v1::resources::moderation::ModerationObject;
use openai_dive::v1::resources::moderation::ModerationParametersBuilder;
use std::sync::Arc;
use tokio::task::JoinSet;
use url::Url;

pub struct Moderator {
    client: Arc<Client>,
}

#[derive(Debug, Clone, Default)]
pub struct Verdict {
    pub flagged: bool,
    pub details: Vec<String>,
}

impl Moderator {
    pub fn new(openai_api_key: String, base_url: Option<Url>) -> Self {
        let mut client = Client::new(openai_api_key);
        if let Some(url) = base_url {
            client.set_base_url(url.as_str());
        }
        Self {
            client: Arc::new(client),
        }
    }

    /// Moderate `content` to avoid indecent text or images.
    ///
    /// Internally, will parse images' urls (including data ones) out of
    /// the provided content, and use OpenAI Moderation API to flag indecent
    /// text or images.
    ///
    /// See: <https://platform.openai.com/docs/guides/moderation>
    pub async fn moderate(&self, content: &str) -> anyhow::Result<Verdict> {
        let image_urls = utils::parse_content(&content);

        let mut image_urls = image_urls.iter();
        let mut tasks = JoinSet::new();
        match image_urls.next() {
            // we have not identified images in the content, so we are only
            // checking text content
            None => {
                let parameters = ModerationParametersBuilder::default()
                    .model("omni-moderation-latest")
                    .input(ModerationInput::Text(content.to_string()))
                    .build()
                    .context("failed to build moderation parameters")?;
                let client = Arc::clone(&self.client);
                tasks.spawn(async move { client.moderations().create(parameters).await });
            }
            Some(image_url) => {
                // we got at least one image, so let's combine it with the text
                // content and send as a multimodal object ...
                let parameters = ModerationParametersBuilder::default()
                    .model("omni-moderation-latest")
                    .input(ModerationInput::MultiModal(vec![
                        ModerationObject::text(content),
                        ModerationObject::image_url(image_url.as_str()),
                    ]))
                    .build()
                    .context("failed to build moderation parameters")?;
                let client = Arc::clone(&self.client);
                tasks.spawn(async move { client.moderations().create(parameters).await });

                // ... and also send the rest of the images (if any);
                // unfortunately, the OpenAI Moderation API will error back if
                // more than 1 image is attached to the multi-modal request,
                // and as of October 2025 there is no option to send an image
                // object specifically, so we are (ab)using a multi-modal option
                while let Some(image_url) = image_urls.next() {
                    let parameters = ModerationParametersBuilder::default()
                        .model("omni-moderation-latest")
                        .input(ModerationInput::MultiModal(vec![
                            // not attching text content to each image to reduce
                            // traffic and token consumption; admittedly, this
                            // may lead to inferior results _if_ the model treats
                            // the textual content as the context for the image
                            // we are asking to check (or vice versa); however,
                            // for now we only want to flag images that are
                            // obviously indecent in any context
                            ModerationObject::image_url(image_url.as_str()),
                        ]))
                        .build()
                        .context("failed to build moderation parameters")?;
                    let client = Arc::clone(&self.client);
                    tasks.spawn(async move { client.moderations().create(parameters).await });
                }
            }
        };

        while let Some(result) = tasks.join_next().await {
            let result = result.context("failed to join moderation task")?;
            dbg!(result);
        }
        Ok(Verdict::default())
    }
}

mod utils {
    pub(super) fn parse_content(content: &str) -> Vec<url::Url> {
        let arena = comrak::Arena::new();
        let root = comrak::parse_document(&arena, content, &comrak::Options::default());
        let mut image_urls = Vec::new();
        for node in root.descendants() {
            match node.data.borrow().value {
                comrak::nodes::NodeValue::Image(ref link) => {
                    if let Ok(url) = url::Url::parse(&link.url) {
                        image_urls.push(url);
                    } else {
                        // TODO: consider singalling back to user
                        continue;
                    };
                }
                comrak::nodes::NodeValue::HtmlInline(ref literal)
                | comrak::nodes::NodeValue::HtmlBlock(comrak::nodes::NodeHtmlBlock {
                    ref literal,
                    ..
                }) => {
                    let fragment = scraper::Html::parse_fragment(literal);
                    for img in
                        fragment.select(&scraper::Selector::parse("img").expect("valid selector"))
                    {
                        if let Some(url) = img
                            .value()
                            .attr("src")
                            .and_then(|src| url::Url::parse(src).ok())
                        {
                            image_urls.push(url);
                        }
                    }
                }
                _ => continue,
            }
        }
        image_urls
    }
}
