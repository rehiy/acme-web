import { Component, HostBinding } from '@angular/core';

@Component({
    selector: 'layout-toast',
    templateUrl: 'index.html',
    styleUrls: ['index.scss']
})
export class LayoutToastComponent {

    public items: Toast[] = [];

    @HostBinding()
    public class = 'toast-container position-fixed top-0 end-0 p-3';

    constructor() {
        this.register();
    }

    public create(toast: Toast) {
        toast.classname = `bg-${toast.type || 'success'} text-light`;
        this.items.push(toast);
    }

    public remove(toast: Toast) {
        this.items = this.items.filter((t) => t !== toast);
    }

    public clear() {
        this.items.splice(0, this.items.length);
    }

    private register() {
        // 处理 js 异常
        window.onerror = (message) => {
            this.create({ type: 'danger', message: String(message) });
        };
        // 处理 promise 未捕获的 rejection
        window.addEventListener('unhandledrejection', e => {
            this.create({ type: 'danger', message: String(e.reason) });
            e.preventDefault && e.preventDefault();
        });
        // 处理 postMessage 信息
        window.addEventListener('message', e => {
            if (e && e.data && e.data.type) {
                console.log(e.data);
                this.create({ type: e.data.type, message: String(e.data.message) });
            }
        });
    }

}

export interface Toast {
    type: string;
    message: string;
    classname?: string;
    delay?: number;
}
