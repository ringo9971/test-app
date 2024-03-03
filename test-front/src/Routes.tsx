import { RouteObject } from "react-router-dom";

import GamePage from "./pages/GamePage";
import LoginPage from "./pages/LoginPage";

const routes: RouteObject[] = [
  {
    children: [
      { path: "/", element: <GamePage /> },
      { path: "/login", element: <LoginPage /> },
    ],
  },
];

export default routes;
