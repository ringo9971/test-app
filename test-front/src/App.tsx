import { useState, useEffect } from "react";
import DeleteIcon from "@mui/icons-material/Delete";
import {
  Box,
  Button,
  TextField,
  TableContainer,
  Table,
  IconButton,
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

  const getUsers = async () => {
    const users = await apiClient.get<User[]>("users");
    setUsers(users);
  };
  useEffect(() => {
    getUsers();
  }, []);

  const createUser = async () => {
    const create_user = {
      name,
    };
    const user = await apiClient.create<CreateUser, User>("users", create_user);
    setUsers((users) => [user, ...users]);
    setName("");
  };

  const deleteUser = async (user_uid: string) => {
    await apiClient.delete(`users/${user_uid}`);
    setUsers((users) => users.filter((u) => u.user_uid != user_uid));
  };

  return (
    <>
      <h1>test-app</h1>
      <Box>
        <TextField value={name} onChange={(e) => setName(e.target.value)} />
        <Button variant="contained" onClick={createUser}>
          追加
        </Button>
      </Box>
      <TableContainer>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>名前</TableCell>
              <TableCell>削除</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {users.map(({ name, user_uid }) => (
              <TableRow key={user_uid}>
                <TableCell>{name}</TableCell>
                <TableCell>
                  <IconButton onClick={() => deleteUser(user_uid)}>
                    <DeleteIcon />
                  </IconButton>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    </>
  );
}

export default App;
