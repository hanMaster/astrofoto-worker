use crate::stuff::config::config;
use crate::stuff::order::Order;
use crate::Result;
use async_mailer::{IntoMessage, Mailer, SmtpMailer};
use log::info;

pub struct Email {
    order: Order,
    order_id: String,
}

impl Email {
    pub fn new(order: Order, order_id: String) -> Self {
        Self { order, order_id }
    }

    pub async fn send(&mut self) -> Result<()> {
        let mail = self.prepare_email_content();
        info!("Sending email with {}", self.order_id);
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
            .html_body(mail)
            .into_message()?;

        mailer.send_mail(message).await?;
        info!("Email sent with {}", self.order_id);
        Ok(())
    }
    fn prepare_email_content(&mut self) -> String {
        let cnt = self.order.files.len() as i32;

        format!(
            r#"
        <!DOCTYPE html>
<html lang="ru">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title></title>
    </head>
    <body style="font-family: sans-serif">
        <h1>
            Получен заказ по
            <span style="color: rgb(12, 193, 67)">WhatsApp</span>
            № <span style="font-weight: 700" id="order-number">{}</span>
        </h1>
        <p>Телефон: <span id="phone">{}</span></p>
        <p>Имя: <span id="name">{}</span></p>

        <p>Бумага: <span id="paper">{} {}</span></p>
        <p>{}шт х {}руб = {}руб</p>
    </body>
</html>
        "#,
            self.order_id,
            self.order.phone,
            self.order.name,
            self.order.paper_size,
            self.order.paper_type,
            cnt,
            self.order.price,
            cnt * self.order.price,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn send_email_test() {
        let order = Order {
            phone: "79147894556".to_string(),
            name: "Иван".to_string(),
            paper_type: "глянцевая".to_string(),
            paper_size: "10x15 - 32руб".to_string(),
            price: 15,
            files: vec!["123".to_string(), "456".to_string(), "789".to_string()],
        };
        let mut mailer = Email::new(order, "WA-18032025-1000".to_string());
        let res = mailer.send().await;
        match res {
            Ok(_) => println!("Sent email"),
            Err(ref e) => println!("Error sending email: {}", e),
        }
        assert!(res.is_ok());
    }
}
