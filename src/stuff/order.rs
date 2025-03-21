use crate::stuff::mailer::Email;
use crate::stuff::state::AppState;
use chrono::Local;
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Order {
    pub phone: String,
    pub name: String,
    pub paper_type: String,
    pub paper_size: String,
    pub files: Vec<String>,
}

pub async fn save_order(state: AppState, order: Order) -> crate::Result<()> {
    let date = Local::now().format("%d%m%Y").to_string();
    let cnt = state
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let order_id = format!("WA-{}-{}", date, cnt);
    info!("New order {} received\n{:#?}", order_id, order);
    let work_dir_str = format!("{}/{}", state.work_dir, order_id);
    let work_dir = Path::new(&work_dir_str);
    fs::create_dir(work_dir).await?;
    let mut file = File::create_new(format!("{}/order.txt", work_dir.display())).await?;

    let payload = format!(
        "Телефон: {:1}{}\nИмя: {:5}{}\nБумага: {:2}{}\nРазмер: {:2}{}",
        "", order.phone, "", order.name, "", order.paper_type, "", order.paper_size
    );

    file.write_all(payload.as_bytes()).await?;
    file.sync_all().await?;

    tokio::spawn(download_files(order, work_dir_str, order_id));

    Ok(())
}

async fn download_files(order: Order, dir: String, order_id: String) -> crate::Result<()> {
    for file_url in &order.files {
        let f = Client::new().get(file_url).send().await?.bytes().await?;

        let f_name = file_url.split('/').last().unwrap();

        let mut file = File::create_new(format!("{}/{}", dir, f_name)).await?;
        file.write_all(&f).await?;
        file.sync_all().await?;
    }
    info!("All files saved to {}", dir);

    let mut mailer = Email::new(order, order_id.clone());
    mailer
        .send()
        .await
        .unwrap_or_else(|e| error!("Error sending email for {}: {}", order_id, e));
    Ok(())
}
