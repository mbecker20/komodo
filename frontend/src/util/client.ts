import axios, { AxiosRequestConfig } from "axios";
import { User } from "@monitor/types";

export default class Client {
  token = localStorage.getItem("access_token") || (import.meta.env.VITE_ACCESS_TOKEN as string || null);

  constructor(private baseURL: string) {
		const params = new URLSearchParams(location.search);
    const token = params.get("token");
    if (token) {
			this.token = token;
      localStorage.setItem("access_token", this.token);
      history.replaceState(
        {},
        "",
        this.baseURL
      );
    }
	}

  async login(username: string, password: string) {
    const jwt: string = await this.post("/login/local", { username, password });
    this.token = jwt;
    localStorage.setItem("access_token", this.token);
    return await this.getUser();
  }

  loginGithub() {
    window.location.replace(`${this.baseURL}/login/github`);
  }

  async signup(username: string, password: string) {
    const jwt: string = await this.post("/signup", { username, password });
    this.token = jwt;
    localStorage.setItem("access_token", this.token);
    return await this.getUser();
  }

  logout() {
    localStorage.removeItem("access_token");
    this.token = null;
  }

  async getUser(): Promise<User | false> {
    // this basically check to see if user is authenticated
    if (this.token) {
      try {
        return await this.get("/user");
      } catch {
        this.logout();
        return false;
      }
    } else {
      return false;
    }
  }

  async get<T = any>(url: string, config?: AxiosRequestConfig) {
    return await axios({
      method: "get",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
      ...config,
    }).then(({ data }) => data as T);
  }

  async post<Data = any>(url: string, data?: Data) {
    return await axios({
      method: "post",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
      data,
    }).then(({ data }) => data);
  }

  async put<Data = any>(url: string, data: Data) {
    return await axios({
      method: "put",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
      data,
    }).then(({ data }) => data);
  }

  async patch<Data = any>(url: string, data: Data) {
    return await axios({
      method: "patch",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
      data,
    }).then(({ data }) => data);
  }

  async delete(url: string) {
    return await axios({
      method: "delete",
      url: this.baseURL + url,
      headers: {
        authorization: `Bearer ${this.token}`,
      },
    }).then(({ data }) => data);
  }
}