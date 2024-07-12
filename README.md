# HostChan Host写入脚本

[HostChan](hhttps://github.com/Zero-Lain/host-chan)
Host写入脚本

## 使用方法

#### 1.准备一个host文件的下载地址
#### 2.修改config/config.toml中的url字段
#### 3.设置定时任务启动脚本

## 构建
```shell
#Needs perl or use cross-rs to build !!!
#win 
cargo build --target i686-pc-windows-msvc --release
cargo build --target x86_64-pc-windows-msvc --release
#linux
cargo build --target i686-unknown-linux-musl --release
cargo build --target x86_64-unknown-linux-musl --release
```