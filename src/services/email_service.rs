use anyhow::{anyhow, Result};
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
};
use std::env;

pub struct EmailService;

impl EmailService {
    pub async fn send(
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<()> {
        let smtp_host =
            env::var("SMTP_HOST")
                .map_err(|_| anyhow!("SMTP_HOST não configurado"))?;

        let smtp_username =
            env::var("SMTP_USERNAME")
                .map_err(|_| anyhow!("SMTP_USERNAME não configurado"))?;

        let smtp_password =
            env::var("SMTP_PASSWORD")
                .map_err(|_| anyhow!("SMTP_PASSWORD não configurado"))?;

        let smtp_from =
            env::var("SMTP_FROM")
                .map_err(|_| anyhow!("SMTP_FROM não configurado"))?;

        let email = Message::builder()
            .from(smtp_from.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())?;

        let credentials =
            Credentials::new(
                smtp_username,
                smtp_password,
            );

        let mailer =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_host)?
                .credentials(credentials)
                .build();

        mailer
            .send(email)
            .await
            .map_err(|err| anyhow!("Erro ao enviar e-mail: {}", err))?;

        Ok(())
    }
}