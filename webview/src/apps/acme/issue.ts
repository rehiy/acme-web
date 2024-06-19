import { Component } from '@angular/core';

import { CaList } from '../../helpers/const/ca-list';
import { Acme, IssueOptions, DnsProvider } from '../../openapi/acme';


@Component({
    selector: 'page-acme-issue',
    templateUrl: 'issue.html'
})
export class AcmeIssueComponent {

    public caList = CaList;

    public dnsList!: DnsProvider;

    public issueForm: IssueOptions = {
        dns: '',
        domain: [''],
        server: 'https://acme-v02.api.letsencrypt.org/directory',
        env: {},
    };

    constructor(private acme: Acme) {
        this.dnsProvider();
    }

    public async dnsProvider() {
        this.dnsList = await this.acme.dnsProvider();
    }

    public async issue() {
        const opts = { ...this.issueForm };
        opts.domain = opts.domain[0].split('\n').map(item => item.trim());
        await this.acme.issue(opts);
    }

}