# Protestify

A companion app for productive.io cucumber-js tests

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Installation

Application is compiled for osx (aarch64), so you can visit the application repository, [releases page](https://github.com/trogulja/protestify/releases) - [latest release](https://github.com/trogulja/protestify/releases/latest). There you can download the disk image (.dmg file) and open it. Installation is done by drag and dropping the app from the disk image to the application folder.

Important! -> because this app isn't signed and doesn't go throught the App store, it will be quarantined when first downloaded. To make it work, we need to unquarantine it first:

```bash
xattr -c /Applications/protestify.app
```

and that's it... once installed, in-app updates won't require quarantine removal, but future reinstallations will.
