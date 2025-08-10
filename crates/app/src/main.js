const { invoke } = window.__TAURI__.core;

let greetInputEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  const result = await invoke("greet", { name: greetInputEl.value });
  console.log(result);
  greetInputEl.value = "";
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
