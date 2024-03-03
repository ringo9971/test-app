import { User, onAuthStateChanged } from "firebase/auth";
import { useEffect, useState } from "react";
import useFirebase from "./useFirebase";

export const useUser = () => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const { auth } = useFirebase();

  useEffect(() => {
    setLoading(true);
    onAuthStateChanged(auth, (user) => {
      setUser(user);
      setLoading(false);
    });
  }, [auth]);

  return { user, loading };
};
