import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import Mousetrap from "mousetrap";

// global: Mousetrap
// access tauri apis over window.__TAURI__
const tauri = window.__TAURI__;

console.log("TAURI: ", tauri);

const container = document.getElementById("md-container")!;

const htmlFromMd: string = await invoke("load_markdown");
if (htmlFromMd !== null) {
    container.innerHTML = htmlFromMd;
}

const stdin = await invoke("is_stdin");
if (!stdin) {
    // @ts-ignore
    const _unlisten = listen("watch", (event: {event: string, payload: string}) => {
        container.innerHTML = event.payload;
    });
    invoke("watch_file");
}

// tauri.tauri.invoke("stream");
//
Mousetrap.bind("j", () => {
    window.scrollBy({
        top: window.innerHeight / 30,
        left: 0,
        behavior: "smooth",
    });
});

Mousetrap.bind("k", () => {
    window.scrollBy({
        top: -window.innerHeight / 30,
        left: 0,
        behavior: "smooth",
    });
});

Mousetrap.bind("g g", () => {
    window.scroll({ top: 0, left: 0, behavior: "smooth" });
});

Mousetrap.bind("G", () => {
    window.scroll({ top: container.scrollHeight, left: 0, behavior: "smooth" });
});

Mousetrap.bind("q", () => {
    invoke("exit");
});

Mousetrap.bind("ctrl+d", () => {
    window.scrollBy({
        top: window.innerHeight / 2,
        left: 0,
        behavior: "smooth",
    });
});

Mousetrap.bind("ctrl+u", () => {
    window.scrollBy({
        top: -window.innerHeight / 2,
        left: 0,
        behavior: "smooth",
    });
});

Mousetrap.bind("-", () => {
    //@ts-ignore
    document.body.style.zoom *= 0.9;
});

Mousetrap.bind("+", () => {
    //@ts-ignore
    document.body.style.zoom *= 1.1;
});

Mousetrap.bind("=", () => {
    //@ts-ignore
    document.body.style.zoom = 1;
});

