import axios from "axios";
import type { UserLogin } from "../../types";

export const login = async (username: string, password: string) => {
  const data: UserLogin = {
    username,
    password,
  };
  try {
    const res = await axios({
      method: "post",
      url: "http://localhost:3000/api/v1/users/login",
      data,
      withCredentials: true,
    });
    if (res.data.status === "success") {
      alert("Loggin in");
      window.setTimeout(() => {
        location.assign("/overview"), 1500;
      });
    }
  } catch (error) {
    alert(error.response.data.message);
  }
};
