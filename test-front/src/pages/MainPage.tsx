import { useState, useEffect } from "react";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import {
  Box,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
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
import ApiClient from "../ApiClient";

interface CreateUser {
  name: string;
}

interface UpdateUser {
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
  const [editUser, setEditUser] = useState<UpdateUser | null>(null);
  const [editUserUid, setEditUserUid] = useState<string | null>(null);

  const handleOpen = (name: string, user_uid: string) => {
    setEditUser({ name });
    setEditUserUid(user_uid);
  };
  const handleClose = () => {
    setEditUser(null);
    setEditUserUid(null);
  };

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

  const updateUser = async () => {
    if (editUser === null || editUserUid === null) return;
    let update_user = await apiClient.update<UpdateUser, User>(
      `users/${editUserUid}`,
      editUser
    );
    setUsers((users) =>
      users.map((user) => {
        if (user.user_uid === update_user.user_uid) {
          return update_user;
        } else {
          return user;
        }
      })
    );
    setEditUser(null);
    setEditUserUid(null);
  };

  const deleteUser = async (user_uid: string) => {
    await apiClient.delete(`users/${user_uid}`);
    setUsers((users) => users.filter((u) => u.user_uid != user_uid));
  };

  return (
    <>
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
              <TableCell>編集</TableCell>
              <TableCell>削除</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {users.map(({ name, user_uid }) => (
              <TableRow key={user_uid}>
                <TableCell>{name}</TableCell>
                <TableCell>
                  <IconButton onClick={() => handleOpen(name, user_uid)}>
                    <EditIcon />
                  </IconButton>
                </TableCell>
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
      <Dialog open={editUser !== null} onClose={handleClose}>
        <DialogTitle>編集</DialogTitle>
        <DialogContent>
          <TextField
            value={editUser?.name}
            onChange={(e) => {
              setEditUser((user) => {
                if (user === null) {
                  return null;
                } else {
                  return {
                    name: e.target.value,
                  };
                }
              });
            }}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose}>キャンセル</Button>
          <Button onClick={updateUser}>保存</Button>
        </DialogActions>
      </Dialog>
    </>
  );
}

export default App;
