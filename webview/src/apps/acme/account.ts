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

    public registerForm: RegisterAccountOptions = {
        server: 'https://acme-v02.api.letsencrypt.org/directory',
        'eab-kid': '',
        'eab-hmac-key': '',
    };

    public updateForm: UpdateAccountOptions = {
        email: '',
    };

    public setDefaultCAForm: SetDefaultCAOptions = {
        server: '',
    };

    constructor(
        private acme: Acme,
        private router: Router
    ) {
        this.getAcmeInfo();
    }

    public async getAcmeInfo() {
        this.info = await this.acme.info();
    }

    public registerAccount() {
        const opts = this.registerForm;
        if (opts['eab-kid'] == '') {
            delete opts['eab-kid'];
            delete opts['eab-hmac-key'];
        }
        return this.acme.registerAccount(opts).then(() => {
            this.router.navigate(['acme/account']);
        });
    }

    public updateAccount() {
        return this.acme.updateAccount(this.updateForm).then(() => {
            this.router.navigate(['acme/account']);
        });
    }

    public setDefaultCA() {
        return this.acme.setDefaultCA(this.setDefaultCAForm).then(() => {
            this.router.navigate(['acme/account']);
        });
    }

}