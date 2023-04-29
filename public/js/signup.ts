import axios from "axios";
import type { NewUser } from "../../types";

export const signup = async (newUser: NewUser) => {
  try {
    const res = await axios({
      method: "post",
      url: "http://localhost:3000/api/v1/signup",
      data: newUser,
      withCredentials: true,
    });
    if (res.data.status === "success") {
      alert("Account created");
      location.assign("overview");
    } else {
      location.assign("/"); //not sure
    }
  } catch (error) {
    console.log(error);
    //must handle these
  }
};
