import { login } from "./login";

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
