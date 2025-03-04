import { invoke } from "@tauri-apps/api/core";
import Mousetrap from "mousetrap";

const container = document.getElementById("md-container")!;

export function keybindings() {
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
        window.scroll({
            top: container.scrollHeight,
            left: 0,
            behavior: "smooth",
        });
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
        //@ts-ignore TODO: check if that works on other platforms than linux
        document.body.style.zoom *= 0.9;
    });

    Mousetrap.bind("+", () => {
        //@ts-ignore TODO: check if that works on other platforms than linux
        document.body.style.zoom *= 1.1;
    });

    Mousetrap.bind("=", () => {
        //@ts-ignore TODO: check if that works on other platforms than linux
        document.body.style.zoom = 1;
    });
}
