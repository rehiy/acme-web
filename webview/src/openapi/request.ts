export class Acme {
    constructor() { }

    public list() {
        const data = { action: 'list' };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    public issue(opts: IssueOptions) {
        const data = { action: 'issue', ...opts };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    // 账号相关接口

    public info() {
        const data = { action: 'info' };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    public registerAccount(opts: RegisterAccountOptions) {
        const data = { action: 'register-account', ...opts };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    public updateAccount(email: string) {
        const data = { action: 'update-account', email };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    public setDefaultCA(server: string) {
        const data = { action: 'set-default-ca', server };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    // 私有工具函数

    private async request(input: string, options: RequestInit = {}) {
        // 设置 Header
        const headers: Record<string, string> = {
            'Content-Type': 'application/json'
        };
        const token = sessionStorage.getItem('token');
        if (token && token.trim().length > 1) {
            headers['Authorization'] = `Bearer ${token}`;
        }
        options.headers = Object.assign(headers, options.headers);
        // 发起 HTTP 请求
        try {
            const response = await fetch(input, options);
            const data = await response.json();
            if (data.Error) {
                if (data.Error.Message) {
                    throw new Error(data.Error.Message);
                }
                throw data.Error;
            }
            if (data.Message) {
                window.postMessage({ message: data.Message, type: 'success' });
            }
            if (response.status < 200 || response.status > 400) {
                throw new Error(response.statusText);
            }
            return data.Payload;
        } catch (error) {
            window.postMessage({ message: error, type: 'danger' });
            throw error;
        }
    }
}

export interface IssueOptions {
    domain: Array<string>;
    dns: string;
}

export interface RegisterAccountOptions {
    server: string;
    "eab-kid"?: string;
    "eab-hmac-key"?: string;
}
