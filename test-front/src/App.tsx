import { useState, useEffect } from "react";
import {
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
} from "@mui/material";
import ApiClient from "./ApiClient";

interface User {
  user_uid: string;
  name: string;
}

function App() {
  const apiClient = new ApiClient();
  const [users, setUsers] = useState<User[]>([]);

  const get = async () => {
    const users = await apiClient.get<User[]>("users");
    setUsers(users);
    console.log(users);
  };
  useEffect(() => {
    get();
  }, []);

  return (
    <>
      <h1>test-app</h1>
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>名前</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {users.map(({ name, user_uid }) => (
              <TableRow key={user_uid}>
                <TableCell>{name}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </>
  );
}

export default App;
