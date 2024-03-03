import axios, { AxiosInstance, AxiosResponse } from "axios";
import { User } from "firebase/auth";

export class ApiClient {
  private axiosInstance: AxiosInstance;
  private user: User | null;

  constructor(user: User | null) {
    this.axiosInstance = axios.create({
      baseURL: import.meta.env.VITE_API_BASE_URL,
    });
    this.user = user;
  }

  private async request<T, R>(
    method: string,
    endpoint: string,
    data?: T
  ): Promise<R> {
    if (!this.user) {
      throw new Error("User is not authenticated.");
    }

    const token = this.user.uid;

    try {
      const response: AxiosResponse<R> = await this.axiosInstance.request({
        method,
        url: endpoint,
        data,
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      return response.data;
    } catch (error) {
      throw new Error(`Failed to fetch data from ${endpoint}: ${error}`);
    }
  }
  async get<T>(endpoint: string): Promise<T> {
    console.log("get");
    return this.request<void, T>("GET", endpoint);
  }

  async post<T, R>(endpoint: string, model: T): Promise<R> {
    return this.request<T, R>("POST", endpoint, model);
  }

  async put<T, R>(endpoint: string, model: T): Promise<R> {
    return this.request<T, R>("PUT", endpoint, model);
  }

  async delete(endpoint: string) {
    return this.request<void, void>("DELETE", endpoint);
  }
}

export default ApiClient;
