import { signOut } from "firebase/auth";
import useFirebase from "./useFirebase";

export const useLogout = () => {
  const { auth } = useFirebase();

  const logout = () => {
    signOut(auth);
  };

  return { logout };
};
