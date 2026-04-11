use stevessr_core::error::{Error, Result};

/// Email message to be sent.
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
    pub reply_to: Option<String>,
    pub headers: Vec<(String, String)>,
}

/// SMTP configuration.
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_address: String,
    pub from_name: String,
    pub use_tls: bool,
}

pub struct EmailSender {
    config: SmtpConfig,
}

impl EmailSender {
    pub fn new(config: SmtpConfig) -> Self {
        Self { config }
    }

    /// Send an email message.
    pub async fn send(&self, message: &EmailMessage) -> Result<()> {
        use lettre::{
            message::{header::ContentType, Mailbox, MultiPart, SinglePart},
            transport::smtp::authentication::Credentials,
            AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
        };

        let from: Mailbox = format!("{} <{}>", self.config.from_name, self.config.from_address)
            .parse()
            .map_err(|e| Error::Internal(format!("invalid from address: {}", e)))?;

        let to: Mailbox = message
            .to
            .parse()
            .map_err(|e| Error::Internal(format!("invalid to address: {}", e)))?;

        let mut email_builder = Message::builder()
            .from(from)
            .to(to)
            .subject(&message.subject);

        if let Some(ref reply_to) = message.reply_to {
            let reply_to_mailbox: Mailbox = reply_to
                .parse()
                .map_err(|e| Error::Internal(format!("invalid reply-to address: {}", e)))?;
            email_builder = email_builder.reply_to(reply_to_mailbox);
        }

        let email = email_builder
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(message.text_body.clone()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(message.html_body.clone()),
                    ),
            )
            .map_err(|e| Error::Internal(format!("failed to build email: {}", e)))?;

        let creds = Credentials::new(
            self.config.username.clone(),
            self.config.password.clone(),
        );

        let mailer: AsyncSmtpTransport<Tokio1Executor> = if self.config.use_tls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.config.host)
                .map_err(|e| Error::Internal(format!("SMTP transport error: {}", e)))?
                .credentials(creds)
                .port(self.config.port)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.config.host)
                .port(self.config.port)
                .credentials(creds)
                .build()
        };

        mailer
            .send(email)
            .await
            .map_err(|e| Error::Internal(format!("failed to send email: {}", e)))?;

        Ok(())
    }

    /// Send a notification email for a post reply.
    pub async fn send_notification(
        &self,
        to_email: &str,
        username: &str,
        topic_title: &str,
        post_excerpt: &str,
        url: &str,
    ) -> Result<()> {
        let html_body = format!(
            "<h3>{} replied to: {}</h3><blockquote>{}</blockquote><p><a href=\"{}\">View the full post</a></p>",
            username, topic_title, post_excerpt, url
        );

        let text_body = format!(
            "{} replied to: {}\n\n{}\n\nView the full post: {}",
            username, topic_title, post_excerpt, url
        );

        let message = EmailMessage {
            to: to_email.to_string(),
            subject: format!("[Reply] {}", topic_title),
            html_body,
            text_body,
            reply_to: None,
            headers: vec![],
        };

        self.send(&message).await
    }
}
