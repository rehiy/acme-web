import { Component } from '@angular/core';

import { Acme, InfoResponse } from '../../openapi/acme';


@Component({
    selector: 'page-welcome',
    templateUrl: 'index.html'
})
export class WelcomeComponent {
    public info!: InfoResponse;

    constructor(private acme: Acme) {
        this.getAcmeInfo();
    }

    public async getAcmeInfo() {
        this.info = await this.acme.info();
    }
}