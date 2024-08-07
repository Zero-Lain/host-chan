/*
 * @Author: Sanka vaeceby2@qq.com
 * @Date: 2024-01-04 11:28:03
 * @LastEditors: Sanka vaeceby2@qq.com
 * @LastEditTime: 2024-07-12 9:43:13
 * @FilePath: \hostChan\src\main.rs
 * @Description:
 * Auto fetch hosts file.
 * Copyright (c) 2024 by vaecebyZ, All Rights Reserved.
 */
use config::{Config, File};
use reqwest;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

const START_PROFIX: &str = "#===HostChanStart===#";
const END_PROFIX: &str = "#===HostChanEnd===#";

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
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "》Unsupported operating system.",
        ))
    }
}

fn append_to_hosts_file(content: &str) -> std::io::Result<()> {
    let hosts_path = Path::new(get_hosts_path()?);
    let file = std::fs::File::open(&hosts_path)?;
    let reader = BufReader::new(file);
    let mut file_lines = reader.lines().collect::<io::Result<Vec<String>>>()?;
    // 查找插入位置
    let start_index = file_lines
        .iter()
        .position(|line| line.trim() == START_PROFIX);
    let end_index = file_lines.iter().position(|line| line.trim() == END_PROFIX);
    let insert_content = content;
    if let (Some(start), Some(end)) = (start_index, end_index) {
        // 删除旧内容
        file_lines.drain(start..=end);
        // 插入内容
        file_lines.insert(start, START_PROFIX.to_string());
        file_lines.insert(start + 1, insert_content.to_string());
        file_lines.insert(start + 2, END_PROFIX.to_string());
    } else {
        // 如果没有找到插入位置，则在文件末尾添加内容
        file_lines.push(START_PROFIX.to_string());
        file_lines.push(insert_content.to_string());
        file_lines.push(END_PROFIX.to_string());
    }

    // 写入文件
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&hosts_path)?;
    for line in file_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("》HostChan《");
    let mut config = Config::new();
    // if file not exit ?
    config.merge(File::with_name("config/config.toml")).unwrap();
    let url = config.get_str("host.url").unwrap();
    if url.is_empty() {
        eprintln!("》Please set url in config/config.toml");
    } else {
        println!("》Fetch hosts file from: {}", url);
        match download_hosts_file(&url).await {
            Ok(content) => match append_to_hosts_file(&content) {
                Ok(()) => println!("》Hosts file updated successfully!"),
                Err(err) => {
                    eprintln!("》Failed to append to hosts file: {}", err);
                    println!("》May be try it in Administrator Or Root...");
                }
            },
            Err(err) => eprintln!("》Failed to download hosts file: {}", err),
        }
    }
    println!("》Press any key to exit...");
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    println!("》Exiting program...");
}
