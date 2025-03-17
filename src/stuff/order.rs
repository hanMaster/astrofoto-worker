use std::path::Path;
use chrono::Local;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::stuff::state::AppState;

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
    let cnt = state.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let work_dir_str = format!("{}/WA-{}-{}", state.work_dir , date, cnt);
    let work_dir = Path::new(&work_dir_str);
    fs::create_dir(work_dir).await?;
    let mut file = File::create_new(format!("{}/order.txt", work_dir.display())).await?;

    let payload = format!(
        "Телефон: {:1}{}\nИмя: {:5}{}\nБумага: {:2}{}\nРазмер: {:2}{}",
        "", order.phone, "", order.name, "", order.paper_type, "", order.paper_size
    );

    file.write_all(payload.as_bytes()).await?;
    file.sync_all().await?;

    tokio::spawn(download_files(order.files, work_dir_str));

    Ok(())
}

async fn download_files(files: Vec<String>, dir: String) -> crate::Result<()> {
    for file_url in files {
        let f = Client::new().get(&file_url).send().await?.bytes().await?;

        let f_name = file_url.split('/').last().unwrap();

        let mut file = File::create_new(format!("{}/{}", dir, f_name)).await?;
        file.write_all(&f).await?;
        file.sync_all().await?;
    }
    println!("All files saved to {}", dir);
    Ok(())
}