import { Component } from '@angular/core';

import { Acme, CertItem } from '../../openapi/acme';


@Component({
    selector: 'page-acme-list',
    templateUrl: 'list.html'
})
export class AcmeListComponent {

    public certList!: CertItem[];

    constructor(private acme: Acme) {
        this.list();
    }

    public async list() {
        this.certList = await this.acme.list();
    }

}