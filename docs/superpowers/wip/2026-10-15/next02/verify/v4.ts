function handleClick() {}
const button = document.querySelector("#submit-btn");
button.addEventListener("click", handleClick);
interface User { name: string; email?: string }
const user: User = { name: "田中" };
console.log(user.email.toUpperCase());
