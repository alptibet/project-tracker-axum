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
      url: "http://localhost:3000/api/v1/login",
      data,
      withCredentials: true,
    });
    if (res.data.status === "success") {
      location.assign("overview");
    } else {
      location.assign("/"); //not sure
    }
  } catch (error) {
    console.log(error);
    //must handle here
    //better view a warning model and loc assign to home again
  }
};
