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

    public info(): Promise<InfoResult> {
        const data = { action: 'info' };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    public registerAccount(opts: RegisterAccountOptions) {
        const data = { action: 'register-account', ...opts };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    public updateAccount(opts: UpdateAccountOptions) {
        const data = { action: 'update-account', ...opts };
        return this.request('/acme', { body: JSON.stringify(data) });
    }

    public setDefaultCA(opts: SetDefaultCAOptions) {
        const data = { action: 'set-default-ca', ...opts };
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

export interface InfoResult {
    ACCOUNT_EMAIL: string;
    DEFAULT_ACME_SERVER: string;
    LE_CONFIG_HOME: string;
}

export interface RegisterAccountOptions {
    server: string;
    'eab-kid'?: string;
    'eab-hmac-key'?: string;
}

export interface UpdateAccountOptions {
    email: string;
}

export interface SetDefaultCAOptions {
    server: string;
}
