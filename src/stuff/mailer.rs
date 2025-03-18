use async_mailer::{IntoMessage, Mailer, SmtpMailer};
use crate::Result;
use crate::stuff::config::config;

pub async fn send_email(mail: String) -> Result<()> {
    println!("Sending email");
    let mailer: SmtpMailer = SmtpMailer::new(
        config().SMTP_SERVER.clone(),
        config().SMTP_PORT,
        async_mailer::SmtpInvalidCertsPolicy::Deny,
        config().SENDER_EMAIL.clone(),
        async_mailer::SecretString::from(config().SENDER_PASS.clone()),
    );

    let message = async_mailer::MessageBuilder::new()
        .from(("From Astrafoto-worker", config().SENDER_EMAIL.as_str()))
        .to(config().RECEIVER_EMAIL.as_str())
        .subject("Новый заказ")
        .text_body(mail)
        .into_message()?;

    mailer.send_mail(message).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn send_email_test() {
        let res = send_email("test email".to_string()).await;
        match res {
            Ok(_) => println!("Sent email"),
            Err(ref e) => println!("Error sending email: {}", e),
        }
        assert!(res.is_ok());
    }
}