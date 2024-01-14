import githubMarkdownCss from "npm:generate-github-markdown-css";
import { minify } from "npm:csso";

const css: string = await githubMarkdownCss({
    light: "light",
    dark: "dark_dimmed",
});

const inserted = css.split("\n").map((line, i) => {
    if (i == 0) {
        line += "\nbody,";
    }
    return line;
}).join("\n");

const minified = minify(inserted);
// Deno.writeTextFile("./ui/style.css", minified.css);

Deno.writeTextFile("./src/style.css", minified.css);
