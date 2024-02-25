import axios, { AxiosInstance, AxiosResponse } from "axios";

class ApiClient {
  private axiosInstance: AxiosInstance;

  constructor() {
    this.axiosInstance = axios.create({
      baseURL: import.meta.env.VITE_API_BASE_URL,
    });
  }

  async get<T>(endpoint: string): Promise<T> {
    try {
      const response: AxiosResponse<T> = await this.axiosInstance.get(endpoint);
      return response.data;
    } catch (error) {
      throw new Error(
        "Failed to fetch data from ${endpoint}: ${errro.message}"
      );
    }
  }

  async create<T, R>(endpoint: string, model: T): Promise<R> {
    try {
      const response: AxiosResponse<R> = await this.axiosInstance.post(
        endpoint,
        model
      );
      return response.data;
    } catch (error) {
      throw new Error(
        "Failed to create data from ${endpoint}, model ${model}: ${errro.message}"
      );
    }
  }

  async update<T, R>(endpoint: string, model: T): Promise<R> {
    try {
      const response: AxiosResponse<R> = await this.axiosInstance.put(
        endpoint,
        model
      );
      return response.data;
    } catch (error) {
      throw new Error(
        "Failed to create data from ${endpoint}, model ${model}: ${errro.message}"
      );
    }
  }

  async delete(endpoint: string) {
    try {
      const response: AxiosResponse = await this.axiosInstance.delete(endpoint);
      return response.data;
    } catch (error) {
      throw new Error(
        "Failed to delete data from ${endpoint}: ${errro.message}"
      );
    }
  }
}

export default ApiClient;
