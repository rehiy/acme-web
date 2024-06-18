import { Component } from '@angular/core';
import { Router } from '@angular/router';

import { CaList } from '../../helpers/const/ca-list';
import { Acme, InfoResult, RegisterAccountOptions, UpdateAccountOptions, SetDefaultCAOptions } from '../../openapi/acme';


@Component({
    selector: 'page-acme-account',
    templateUrl: 'account.html'
})
export class AcmeAccountComponent {
    public caList = CaList;

    public info!: InfoResult;

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

    constructor(
        private acme: Acme,
        private router: Router
    ) {
        this.getAcmeInfo();
    }

    public async getAcmeInfo() {
        this.info = await this.acme.info();
        this.updateAccountForm.email = this.info.ACCOUNT_EMAIL;
        this.setDefaultCAForm.server = this.info.DEFAULT_ACME_SERVER;
    }

    public setDefaultCA() {
        return this.acme.setDefaultCA(this.setDefaultCAForm).then(() => {
            this.router.navigate(['acme/account']);
        });
    }

    public updateAccount() {
        return this.acme.updateAccount(this.updateAccountForm).then(() => {
            this.router.navigate(['acme/account']);
        });
    }

    public registerAccount() {
        const opts = this.registerAccountForm;
        opts['eab-kid'] || delete opts['eab-kid'];
        opts['eab-hmac-key'] || delete opts['eab-hmac-key'];
        return this.acme.registerAccount(opts).then(() => {
            this.router.navigate(['acme/account']);
        });
    }

}