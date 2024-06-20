import { Injectable } from '@angular/core';

import { HttpClient } from './helper';


@Injectable({
    providedIn: 'root'
})
export class Acme extends HttpClient {

    constructor() {
        super();
    }

    // 证书接口

    public list() {
        const body = JSON.stringify({});
        return this.request('/acme/list', { body });
    }

    public issue(opts: IssueOptions) {
        const body = JSON.stringify(opts);
        return this.request('/acme/issue', { body });
    }

    // 账号相关接口

    public info(): Promise<AcmeInfo> {
        const body = JSON.stringify({});
        return this.request('/acme/info', { body });
    }

    public registerAccount(opts: RegisterAccountOptions) {
        const body = JSON.stringify(opts);
        return this.request('/acme/register-account', { body });
    }

    public updateAccount(opts: UpdateAccountOptions) {
        const body = JSON.stringify(opts);
        return this.request('/acme/update-account', { body });
    }

    public setDefaultCA(opts: SetDefaultCAOptions) {
        const body = JSON.stringify(opts);
        return this.request('/acme/set-default-ca', { body });
    }

    // 扩展数据接口

    public caAccount(): Promise<CaAccount[]> {
        const body = JSON.stringify({});
        return this.request('/acme/ca-account', { body });
    }

    public dnsProvider(): Promise<DnsProvider> {
        const body = JSON.stringify({});
        return this.request('/acme/dns-provider', { body });
    }

}

// 证书数据类型

export interface IssueOptions {
    dns: string;
    domain: Array<string>;
    server: string;
    env: Record<string, string>;
}

// 账号相关类型

export interface AcmeInfo {
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

// 扩展数据类型

export interface CaAccount {
    ACCOUNT_URL: string;
    CA_EAB_KEY_ID?: string;
    CA_EAB_HMAC_KEY?: string;
    CA_EMAIL: string;
    CA_KEY_HASH: string;
}

export interface DnsProvider {
    [key: string]: Array<string>;
}
