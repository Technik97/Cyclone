import axios from "axios";
import qs from "qs";
import { sign_up_url } from './urls'

export interface signInRequest {
  username: string,
  email: string,
  password: string
}

export const sign_up = async (username: string, email: string, password: string) => {
    let url = sign_up_url;
    const data: signInRequest = {
      'username': username,
      'email': email,
      'password': password,
    }

    const options = {
      method: 'POST',
      headers: { 'content-type': 'application/x-www-form-urlencoded' },
      data: qs.stringify(data),
      url,
  };
  axios(options);
}
