# Acme.web

## 接口信息

- **POST** `/acme`

```json
{
    "action": "acme.sh 子命令，如 list、info、issue 等",
    "env_xxxx": "执行子命令时要设置的环境变量，xxxx 将作为环境变量名",
    "domain": ["待签域名1", "待签域名2", "这是一个数组"],
    "其它参数": "键名使用长参名称（不添加--），每组键值对应一组参数和值"
}
```

## 开发配置

- 安装 `rust` 开发环境 和 `acme.sh`

- 从源码启动 `PATH=$PATH:~/.acme.sh cargo run`

### 临时忽略代理配置

`git update-index --assume-unchanged proxy.conf.json`
