import { useState, useEffect } from "react";
import {
  Box,
  Button,
  TextField,
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
} from "@mui/material";
import ApiClient from "./ApiClient";

interface CreateUser {
  name: string;
}

interface User {
  user_uid: string;
  name: string;
}

function App() {
  const apiClient = new ApiClient();
  const [users, setUsers] = useState<User[]>([]);
  const [name, setName] = useState<string>("");

  const get = async () => {
    const users = await apiClient.get<User[]>("users");
    setUsers(users);
  };
  useEffect(() => {
    get();
  }, []);

  const create = async () => {
    const create_user = {
      name,
    };
    const user = await apiClient.create<CreateUser, User>("users", create_user);
    setUsers((users) => [user, ...users]);
    setName("");
  };

  return (
    <>
      <h1>test-app</h1>
      <Box>
        <TextField value={name} onChange={(e) => setName(e.target.value)} />
        <Button variant="contained" onClick={create}>
          追加
        </Button>
      </Box>
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
