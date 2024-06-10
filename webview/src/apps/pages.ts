import { LoginGuard } from '../helpers/login-guard';

import { WelcomeComponent } from './welcome';
import { Alert404Component } from './alert/404';


export const AppComponents = [
    WelcomeComponent,
    Alert404Component,

];

//////////////////////////////////////////////////////////////////

import { Routes } from '@angular/router';

export const AppRoutes: Routes = [
    { path: 'welcome', component: WelcomeComponent },

    { path: '', redirectTo: 'welcome', pathMatch: 'full' },
    { path: '**', component: Alert404Component },
];
