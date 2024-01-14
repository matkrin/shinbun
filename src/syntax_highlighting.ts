import { common, createStarryNight } from "@wooorm/starry-night";
import { toDom } from "hast-util-to-dom";

export async function highlightCodeBlocks() {
    const starryNight = await createStarryNight(common);
    const prefix = "language-";

    const codeNodes = Array.from(document.body.querySelectorAll("code"));

    for (const node of codeNodes) {
        const className = Array.from(node.classList).find((d) =>
            d.startsWith(prefix)
        );
        if (!className) continue;
        const scope = starryNight.flagToScope(className.slice(prefix.length));
        if (!scope) continue;
        const tree = starryNight.highlight(node.textContent!, scope);
        node.replaceChildren(toDom(tree, { fragment: true }));
    }
}
