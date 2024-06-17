# Acme.web

## 接口信息

- **POST** `/acme`

```json
{
    "action": "acme.sh 子命令，如 list、info、issue 等",
    "env_xxxx": "执行子命令时要设置的环境变量，xxxx 将作为环境变量名",
    "其它参数": "其它参数的值"
}
```

## 开发配置

- 安装 `rust` 开发环境 和 `acme.sh`

- 从源码启动 `PATH=$PATH:~/.acme.sh cargo run`

### 临时忽略代理配置

`git update-index --assume-unchanged proxy.conf.json`
