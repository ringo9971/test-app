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
}

export default ApiClient;
