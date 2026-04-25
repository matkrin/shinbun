import githubMarkdownCss from "npm:generate-github-markdown-css";
import { minify } from "npm:csso";

const css = await githubMarkdownCss({
    light: "light",
    dark: "dark_dimmed",
});

const lightMatch = css.match(
    /@media \(prefers-color-scheme: light\)[\s\S]*?--bgColor-default:\s*([^;]+);/
);

const darkMatch = css.match(
    /@media \(prefers-color-scheme: dark\)[\s\S]*?--bgColor-default:\s*([^;]+);/
);

const lightBg = lightMatch?.[1]?.trim() ?? "#ffffff";
const darkBg = darkMatch?.[1]?.trim() ?? "#212830";

const injected = `
:root {
  --bgColor-default: ${lightBg};
}

@media (prefers-color-scheme: dark) {
  :root {
    --bgColor-default: ${darkBg};
  }
}

html, body {
  margin: 10px;
  background: var(--bgColor-default);
}
`;

const finalCss = injected + "\n" + css;
const minified = minify(finalCss);
await Deno.writeTextFile("./src/style.css", minified.css);
