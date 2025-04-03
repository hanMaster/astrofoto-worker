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
    pub price: i32,
    pub files: Vec<String>,
}

pub async fn save_order(state: AppState, order: Order) -> crate::Result<()> {
    let date = Local::now().format("%d%m%Y").to_string();
    let cnt = state
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let order_id = format!("WA-{}-{}", date, cnt);
    info!("New order {} received\n{:#?}", order_id, order);
    let mut work_dir_str = format!("{}/{}", state.work_dir, order_id);
    let mut work_dir = Path::new(&work_dir_str);
    match fs::create_dir(work_dir).await {
        Ok(_) => {}
        Err(e) => {
            error!("Error creating dir: {}", e);
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e.into());
            } else {
                work_dir_str = format!("{}/{}_1", state.work_dir, order_id);
                work_dir = Path::new(&work_dir_str);
                let res = fs::create_dir(work_dir).await;
                if let Err(e) = res {
                    error!("Error creating dir: {}", e);
                }
            }
        }
    }

    let cnt = order.files.len() as i32;

    let mut file = File::create_new(format!(
        "{}/_{cnt}шт_{}_{}_{}руб.txt",
        work_dir.display(),
        order.paper_size,
        order.paper_type,
        cnt * order.price
    ))
    .await?;

    let payload = format!(
        "Телефон: {:1}{}\nИмя: {:5}{}\nБумага: {:2}{}\nРазмер: {:2}{}\nКоличество:  {} шт\n\nИтого: {}руб.",
        "",
        order.phone,
        "",
        order.name,
        "",
        order.paper_type,
        "",
        order.paper_size,
        cnt,
        cnt * order.price
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
