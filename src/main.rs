/*
 * @Author: Sanka vaeceby2@qq.com
 * @Date: 2024-01-04 11:28:03
 * @LastEditors: Sanka vaeceby2@qq.com
 * @LastEditTime: 2024-01-04 12:24:53
 * @FilePath: \hostChan\src\main.rs
 * @Description: 
 * Auto fetch hosts file.
 * Copyright (c) 2024 by vaecebyZ, All Rights Reserved. 
 */
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use config::{Config, File};
use reqwest;

async fn download_hosts_file(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

fn get_hosts_path() -> std::io::Result<&'static str> {
    if cfg!(target_os = "windows") {
        Ok("C:\\Windows\\System32\\drivers\\etc\\hosts")
    } else if cfg!(target_os = "macos") || cfg!(target_os = "linux") {
        Ok("/etc/hosts")
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported operating system"))
    }
}

fn append_to_hosts_file(content: &str) -> std::io::Result<()> {
    let hosts_path = Path::new(get_hosts_path()?); 
    let mut file = OpenOptions::new().append(true).open(hosts_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut config = Config::new();
    config.merge(File::with_name("config/config.toml")).unwrap();
    let url = config.get_str("host.url").unwrap();
    println!("url: {}", url);
    if url.is_empty(){
        eprintln!("Please set url in config/config.toml");
        return;
    }
    match download_hosts_file(&url).await {
        Ok(content) => {
            match append_to_hosts_file(&content) {
                Ok(()) => println!("Hosts file updated successfully!"),
                Err(err) => eprintln!("Failed to append to hosts file: {}", err),
            }
        },
        Err(err) => eprintln!("Failed to download hosts file: {}", err),
    }
}