import { CanActivateFn } from '@angular/router';


export const LoginGuard: CanActivateFn = async () => {

   const isLogin = false;

   isLogin || location.assign('/#/welcome');
   return isLogin;

};
