import { Component } from '@angular/core';

import { Acme, AcmeInfo, CaAccount } from '../../openapi/acme';


@Component({
    selector: 'page-welcome',
    templateUrl: 'index.html'
})
export class WelcomeComponent {

    public info!: AcmeInfo;

    public caAccounts!: CaAccount[];

    constructor(private acme: Acme) {
        this.getAcmeInfo();
        this.getCaAccount();
    }

    public async getAcmeInfo() {
        this.info = await this.acme.info();
    }

    public async getCaAccount() {
        this.caAccounts = await this.acme.caAccount();
        console.log(this.caAccounts);
    }

}