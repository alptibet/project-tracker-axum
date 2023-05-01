import type { NewUser } from "../../types";
import { login, signup } from "./auth";

const loginForm = document.querySelector(".form-login");
if (loginForm) {
  loginForm.addEventListener("submit", (e) => {
    e.preventDefault();
    const username = (<HTMLInputElement>document.getElementById("username"))
      .value;
    const password = (<HTMLInputElement>document.getElementById("password"))
      .value;
    login(username, password);
  });
}

const signupForm = document.querySelector(".form-signup");
if (signupForm) {
  signupForm.addEventListener("submit", (e) => {
    e.preventDefault();
    const newUser: NewUser = {
      name: (<HTMLInputElement>document.getElementById("name")).value,
      surname: (<HTMLInputElement>document.getElementById("surname")).value,
      username: (<HTMLInputElement>document.getElementById("username")).value,
      email: (<HTMLInputElement>document.getElementById("email")).value,
      password: (<HTMLInputElement>document.getElementById("password")).value,
      passwordConfirm: (<HTMLInputElement>document.getElementById("confirm"))
        .value,
    };
    signup(newUser);
  });
}
