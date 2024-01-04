/*
 * @Author: Sanka vaeceby2@qq.com
 * @Date: 2024-01-04 11:28:03
 * @LastEditors: Sanka vaeceby2@qq.com
 * @LastEditTime: 2024-01-04 12:07:10
 * @FilePath: \hostChan\src\main.rs
 * @Description: 
 * 
 * Copyright (c) 2024 by vaecebyZ, All Rights Reserved. 
 */
use std::fs::OpenOptions;
use std::io::{Write};
use std::path::Path;
use reqwest;

async fn download_hosts_file(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

fn append_to_hosts_file(content: &str) -> std::io::Result<()> {
    // let hosts_path = Path::new("/etc/hosts"); // Linux上的hosts文件路径
    let hosts_path = Path::new("C:\\Windows\\System32\\drivers\\etc\\hosts"); // Windows上的hosts文件路径
    // let hosts_path = Path::new("C:\\Users\\sanka\\Desktop\\lain\\hostChan\\hosts");
    let mut file = OpenOptions::new().append(true).open(hosts_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let url = "https://hosts.sanka.sbs";
    match download_hosts_file(url).await {
        Ok(content) => {
            match append_to_hosts_file(&content) {
                Ok(()) => println!("Hosts file updated successfully!"),
                Err(err) => eprintln!("Failed to append to hosts file: {}", err),
            }
        },
        Err(err) => eprintln!("Failed to download hosts file: {}", err),
    }
}