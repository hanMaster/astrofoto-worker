use async_mailer::{IntoMessage, Mailer, SmtpMailer};
use crate::Result;

pub async fn send_email(mail: String)-> Result<()> {
    println!("Sending email");
    let mailer: SmtpMailer = SmtpMailer::new(
        "smtp.yandex.ru".into(),
        465,
        async_mailer::SmtpInvalidCertsPolicy::Deny,
        "<account>".into(),
        async_mailer::SecretString::from("")
    );

    let message = async_mailer::MessageBuilder::new()
        .from(("From Astrafoto-worker", "<email>"))
        .to("<email>")
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