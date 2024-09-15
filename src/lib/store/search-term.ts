import { writable } from "svelte/store";

const searchTerm = writable('');

export default searchTerm;
