const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetTextAreaEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  const result = await invoke("greet", {
    title: greetInputEl.value,
    body: greetTextAreaEl.value,
  });
  greetInputEl.value = "";
  greetTextAreaEl.value = "";
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetTextAreaEl = document.querySelector("#greet-textarea");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
