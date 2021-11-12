import { makeAutoObservable } from 'mobx';
import { LoginRequest } from '../services/auth';
import { AuthService } from '../services/auth';
import jwtDecode from 'jwt-decode';

export class AuthStore {
  private authenticated: boolean = false;

  constructor(private readonly authService: AuthService) {
    makeAutoObservable(this);
    this.authenticated = !!this.accessToken;
  }

  login = async (loginRequest: LoginRequest) => {
    try {
      const tokenPayloadDto = await this.authService.login(loginRequest);
      localStorage.setItem('access_token', tokenPayloadDto.access_token); // TODO check it
      this.setAuthenticated(true);
    } catch (err) {
      this.setAuthenticated(false);
    }
  }

  logout = () => {
      localStorage.removeItem('access_token');
      this.setAuthenticated(false);
      window.location.reload();      
  }

  private setAuthenticated(authenticated: boolean) {
    this.authenticated = authenticated;
  }

  get accessToken() {
    return localStorage.getItem('access_token');
  }

  get isAuthenticated() {
    return this.authenticated;
  }

  // TODO: remove it
  setDummyAuthenticated = () => {
    this.setAuthenticated(true);
  }

  get currentUser() {    
    const token = localStorage.getItem('access_token');
    if (!!token) {
        return jwtDecode(token);
    }    
    return 'no user'; // TODO  
  }
}