import { Component } from '@angular/core';

import { CaList } from '../../helpers/const/ca-list';
import { Acme, SetDefaultCAOptions, UpdateAccountOptions, RegisterAccountOptions } from '../../openapi/acme';


@Component({
    selector: 'page-acme-account',
    templateUrl: 'account.html'
})
export class AcmeAccountComponent {

    public caList = CaList;

    public setDefaultCAForm: SetDefaultCAOptions = {
        server: '',
    };

    public updateAccountForm: UpdateAccountOptions = {
        email: '',
    };

    public registerAccountForm: RegisterAccountOptions = {
        server: 'https://acme-v02.api.letsencrypt.org/directory',
        'eab-kid': '',
        'eab-hmac-key': '',
    };

    constructor(private acme: Acme) {
        this.getAcmeInfo();
    }

    public async getAcmeInfo() {
        const info = await this.acme.info();
        this.updateAccountForm.email = info.ACCOUNT_EMAIL;
        this.setDefaultCAForm.server = info.DEFAULT_ACME_SERVER;
    }

    public async setDefaultCA() {
        await this.acme.setDefaultCA(this.setDefaultCAForm);
        this.getAcmeInfo();
    }

    public async updateAccount() {
        await this.acme.updateAccount(this.updateAccountForm);
    }

    public async registerAccount() {
        const opts = this.registerAccountForm;
        opts['eab-kid'] || delete opts['eab-kid'];
        opts['eab-hmac-key'] || delete opts['eab-hmac-key'];
        await this.acme.registerAccount(opts);
    }

}