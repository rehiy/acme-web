import { Component, Input } from '@angular/core';


@Component({
    selector: 'layout-footer',
    templateUrl: 'index.html',
    styleUrls: ['index.scss']
})
export class LayoutFooterComponent {

    public collapse = false;

    @Input()
    public set title(val: string) {
        document.title = (val ? val + ' - ' : '') + 'Acme.web';
    }

}
