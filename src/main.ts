import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

import { highlightCodeBlocks } from "./syntax_highlighting";
import { keybindings } from "./keybindings";

const container = document.getElementById("md-container")!;

async function loadMarkdown() {
    const htmlFromMd: string | null = await invoke("load_markdown");
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

async function listenSync() {
    // @ts-ignore
    const _unlisten = listen(
        "sync",
        (event: { event: string; payload: string }) => {
            container.innerHTML = event.payload;
            highlightCodeBlocks();
        },
    );
    invoke("start_sync");
}

const sync: boolean = await invoke("is_sync");
if (sync) {
    listenSync();
} else {
    loadMarkdown();
    const stdin: boolean = await invoke("is_stdin");
    if (!stdin) {
        watchMarkdown();
    }
}

keybindings();
