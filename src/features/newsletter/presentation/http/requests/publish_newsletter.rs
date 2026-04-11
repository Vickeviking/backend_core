use crate::features::newsletter::application::dto::PublishNewsletterIssueCommand;

#[derive(serde::Deserialize)]
pub struct PublishNewsletterFormData {
    title: String,
    text_content: String,
    html_content: String,
    idempotency_key: String,
}

impl PublishNewsletterFormData {
    pub fn into_parts(self) -> (PublishNewsletterIssueCommand, String) {
        (
            PublishNewsletterIssueCommand {
                title: self.title,
                text_content: self.text_content,
                html_content: self.html_content,
            },
            self.idempotency_key,
        )
    }
}
