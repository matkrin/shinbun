import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

import { highlightCodeBlocks } from "./syntax_highlighting";
import { keybindings } from "./keybindings";

const container = document.getElementById("md-container")!;

async function loadMarkdown() {
    const htmlFromMd: string = await invoke("load_markdown");
    if (htmlFromMd !== null) {
        container.innerHTML = htmlFromMd;
    }
    highlightCodeBlocks();
}

async function watchMarkdown() {
    // @ts-ignore
    const _unlisten = listen(
        "watch",
        (event: { event: string; payload: string }) => {
            container.innerHTML = event.payload;
            highlightCodeBlocks();
        },
    );
    invoke("watch_file");
}

// tauri.tauri.invoke("stream");
//
loadMarkdown();
const stdin = await invoke("is_stdin");
if (!stdin) {
    watchMarkdown();
}
keybindings();
