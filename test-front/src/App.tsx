import { useRoutes } from "react-router-dom";
import routes from "./Routes";
import TopBar from "./TopBar";
import useFirebase from "./hooks/useFirebase";
import { useUser } from "./hooks/useUser";
import { ApiClientProvider } from "./lib/ApiClientContext";

function App() {
  const routing = useRoutes(routes);
  const { user, loading } = useUser();
  useFirebase();

  return (
    <>
      <ApiClientProvider user={user}>
        <>
          <TopBar />
          {!loading && <>{routing}</>}
        </>
      </ApiClientProvider>
    </>
  );
}

export default App;
