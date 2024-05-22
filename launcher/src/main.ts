import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";

async function load(callback: Function) {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: "Priedes koda fails",
        extensions: ["pr"],
      },
    ],
  });
  if (selected === null) {
  } else {
    console.log(selected);
    callback(selected);
  }
}

async function runfile(filename: string) {
  invoke('runfile', { name: filename }).then((response) => console.log(response));
}

window.addEventListener("DOMContentLoaded", () => {
  document.addEventListener("contextmenu", (event) => event.preventDefault());
  document.getElementById("openfile")?.addEventListener("click", (e) => {
    e.preventDefault();
    load(
      //@ts-ignore
      (a: string) => (document.getElementById("selectedfile").innerHTML = a)
    );
  });
  document.getElementById("runfile")?.addEventListener("click", (e) => {
    e.preventDefault();
    //@ts-ignore
    runfile(document.getElementById("selectedfile").innerHTML);
  });
});
