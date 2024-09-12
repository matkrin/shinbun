# shinbun

Small markdown reader/previewer with the same appearance as github. The
preview gets automatically updated on every file write/save.

## Build

In project root:
```bash
npm install
npm build
```

In src-tauri:
```bash
carog build --release --features=custom-protocol
```

## Usage

```bash
$ shinbun <markdown-file>
```

## Keybindings

| Keys                         | Action                  |
| ---------------------------- | ----------------------- |
| <kbd>k</kbd>                 | Scroll up               |
| <kbd>j</kbd>                 | Scroll down             |
| <kbd>ctrl</kbd>+<kbd>u</kbd> | Scroll up half a page   |
| <kbd>ctrl</kbd>+<kbd>d</kbd> | Scroll down half a page |
| <kbd>gg</kbd>                | Go to top               |
| <kbd>G</kbd>                 | Go to bottom            |
| <kbd>+</kbd>                 | Zoom in                 |
| <kbd>-</kbd>                 | Zoom out                |
| <kbd>=</kbd>                 | Reset zoom              |
