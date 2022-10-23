import { writable } from "svelte/store";

interface Options {
    size: number;
    win: number;

}

const storedOptions = localStorage.getItem('options')

export const options = writable<Options>(JSON.parse(storedOptions) || { size: 3, win: 3 });
options.subscribe(value => {
    localStorage.setItem('options', JSON.stringify(value))
});

