import { memo, useState } from "react";
import useFirebase from "../hooks/useFirebase";
import {
  Box,
  Button,
  FormControl,
  Input,
  InputLabel,
  Typography,
} from "@mui/material";
import { signInWithEmailAndPassword } from "firebase/auth";
import { useNavigate } from "react-router-dom";
import { FirebaseError } from "firebase/app";

const AuthLogin = () => {
  const { auth } = useFirebase();

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const navigate = useNavigate();

  const signInEmail = async () => {
    try {
      await signInWithEmailAndPassword(auth, email, password);
      navigate("/");
    } catch (error) {
      if (error instanceof FirebaseError) {
        let errorMessage = "";
        switch (error.code) {
          case "auth/invalid-login-credentials":
            errorMessage = "パスワードまたはメールアドレスが間違っています";
            break;
          case "auth/invalid-email":
            errorMessage = "メールアドレスの形式が間違っています";
            break;
          default:
            errorMessage = "エラーが発生しました";
        }
        setErrorMessage(errorMessage);
      }
    }
  };

  return (
    <Box display="flex" flexDirection="column" alignItems="center" mt={2}>
      <Box display="flex" flexDirection="column" alignItems="flex-start">
        <FormControl sx={{ mt: 2 }}>
          <InputLabel>メールアドレス</InputLabel>
          <Input value={email} onChange={(e) => setEmail(e.target.value)} />
        </FormControl>
        <FormControl sx={{ mt: 2 }}>
          <InputLabel htmlFor="password">パスワード</InputLabel>
          <Input
            id="password"
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </FormControl>
        <Button variant="contained" onClick={signInEmail} sx={{ mt: 1 }}>
          ログイン
        </Button>
        {errorMessage && <Typography sx={{ mt: 1 }}>{errorMessage}</Typography>}
      </Box>
    </Box>
  );
};

const NamedAuthLogin = memo(AuthLogin);
export default NamedAuthLogin;
