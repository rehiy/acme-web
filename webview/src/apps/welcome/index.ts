import { Component } from '@angular/core';


@Component({
    selector: 'page-welcome',
    templateUrl: 'index.html',
    styleUrls: ['index.scss']
})
export class WelcomeComponent {

    public islogin = false;
    public upgrade = {
        url: '', version: ''
    };

    public formdata = {
        token: '',
    };

    constructor() {
        this.checkLogin();
        // 获取会话存储的令牌
        this.formdata.token = sessionStorage.getItem('token') || '';
    }

    public submitForm() {
        sessionStorage.setItem('token', this.formdata.token);
        location.reload();
    }

    public async checkLogin() {
    }

    public compareVersions(v1: string, v2: string) {
        const v1s = v1.replace(/^[a-zA-Z]+/, '').split('.');
        const v2s = v2.replace(/^[a-zA-Z]+/, '').split('.');
        const len = Math.min(v1s.length, v2s.length);
        // 逐级比较
        for (let i = 0; i < len; i++) {
            const n1 = parseInt(v1s[i], 10);
            const n2 = parseInt(v2s[i], 10);
            if (n1 > n2) {
                return 1;
            }
            if (n1 < n2) {
                return -1;
            }
        }
        // 版本号长度不同
        if (v1s.length > v2s.length) {
            return 1;
        }
        if (v1s.length < v2s.length) {
            return -1;
        }
        return 0;
    }

}