const {invoke} = window.__TAURI__.core;
const {listen} = window.__TAURI__.event;

document.getElementById('blah').addEventListener('click', () => {
    alert('blah click');
})