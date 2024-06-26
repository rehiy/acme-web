import { LoginGuard } from '../helpers/login-guard';

import { WelcomeComponent } from './welcome';
import { Error404Component } from './message/404';

import { AcmeAccountComponent } from './acme/account';
import { AcmeIssueComponent } from './acme/issue';
import { AcmeListComponent } from './acme/list';


export const AppComponents = [
    WelcomeComponent,
    Error404Component,

    AcmeAccountComponent,
    AcmeIssueComponent,
    AcmeListComponent,

];

//////////////////////////////////////////////////////////////////

import { Routes } from '@angular/router';

export const AppRoutes: Routes = [
    { path: 'welcome', component: WelcomeComponent },

    { path: 'acme/account', component: AcmeAccountComponent },
    { path: 'acme/issue', component: AcmeIssueComponent },
    { path: 'acme/list', component: AcmeListComponent },


    { path: '', redirectTo: 'welcome', pathMatch: 'full' },
    { path: '**', component: Error404Component },
];
