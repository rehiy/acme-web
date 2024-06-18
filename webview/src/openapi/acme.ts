import { Injectable } from '@angular/core';

import { HttpClient } from './helper';


@Injectable({
    providedIn: 'root'
})
export class Acme extends HttpClient {
    constructor() {
        super();
    }

    public list() {
        const data = { action: 'list' };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    public issue(opts: IssueOptions) {
        const data = { action: 'issue', ...opts };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    // 账号相关接口

    public info(): Promise<InfoResponse> {
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
}

export interface KvData {
    [key: string]: string;
}

export interface IssueOptions {
    dns: string;
    domain: Array<string>;
}

export interface InfoResponse {
    ACCOUNT_EMAIL: string;
    DEFAULT_ACME_SERVER: string;
    LE_CONFIG_HOME: string;
}

export interface RegisterAccountOptions {
    server: string;
    "eab-kid"?: string;
    "eab-hmac-key"?: string;
}
