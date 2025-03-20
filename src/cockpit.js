// const {invoke} = window.__TAURI__.core;
// const {listen} = window.__TAURI__.event;
const {store, load} = window.__TAURI__.store;

const smartActionContainer = document.getElementById('smart-actions-container');

// https://v2.tauri.app/reference/javascript/store/#load

window.addEventListener("DOMContentLoaded", async () => {
    const store = await load('store.json', {autoSave: false});
    const storedValues = await store.length();
    // await store.entries.forEach(entry => {
    //     const {key, value} = entry;
    //     console.log(`${key}: ${JSON.stringify(value)}`);
    //     const div = document.createElement('div');
    //     div.textContent = `${key}: ${JSON.stringify(value)}`;
    //     smartActionContainer.appendChild(div);
    // })
    const storeData = await store.entries();
    console.log(storeData);

    storeData.forEach(([key, obj]) => {
        console.log("key:", key);
        console.log("value:", obj);
    });

    if (storedValues > 0) {
        const k = await store.get('some-key');
        console.log(k);
        // await store.set('some-key', {value: 5});
        // await store.save();
    }
});