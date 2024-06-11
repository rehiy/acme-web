# Acme.web

## 开发配置

- 安装 `rust` 开发环境 和 `acme.sh`

- 从源码启动 `PATH=$PATH:~/.acme.sh cargo run`
  
### 临时忽略代理配置

`git update-index --assume-unchanged proxy.conf.json`

## 接口列表

- **POST** `/acme/info` 获取 acme.sh 配置信息

- **POST** `/acme/list` 获取证书列表
