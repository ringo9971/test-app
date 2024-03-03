import { User } from "firebase/auth";
import { createContext, useContext } from "react";

import { ApiClient } from "../api/ApiClient";

interface ApiClientContextProps {
  apiClient: ApiClient;
}

const ApiClientContext = createContext<ApiClientContextProps | null>(null);

export const ApiClientProvider = ({
  user,
  children,
}: {
  user: User | null;
  children: JSX.Element;
}) => {
  const apiClient = new ApiClient(user);

  return (
    <ApiClientContext.Provider value={{ apiClient }}>
      {children}
    </ApiClientContext.Provider>
  );
};

export const useApiClient = () => {
  const context = useContext(ApiClientContext);
  if (!context) {
    throw new Error(
      "useApiClientContext must be used within a ApiCLientProvider"
    );
  }
  return context;
};
