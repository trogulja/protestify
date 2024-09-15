import { writable } from "svelte/store";

const isUpdateAvailable = writable(false);

export default isUpdateAvailable;
