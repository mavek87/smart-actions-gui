// const {invoke} = window.__TAURI__.core;
// const {listen} = window.__TAURI__.event;
const {store, load} = window.__TAURI__.store;

const smartActionContainer = document.getElementById('smart-actions-container');

// https://v2.tauri.app/reference/javascript/store/#load

let divEventListeners = [];

window.addEventListener("DOMContentLoaded", async () => {
    console.log(`cleaning ${divEventListeners?.length || 0} listeners`);
    divEventListeners.forEach(eventListener => {
        console.log("cleaning eventListener for id: " + eventListener.elementId);
        eventListener.elementInstance.removeEventListener('div', eventListener.listenerFn);
    });
    divEventListeners = [];

    const store = await load('store.json', {autoSave: false});

    const storeData = await store.entries();

    const maxElementsPerRow = 3;
    let counterElementsInRow = 0;
    let divWithGrid;
    storeData.forEach(([key, obj]) => {
        if (counterElementsInRow % maxElementsPerRow === 0) {
            divWithGrid = document.createElement("div");
            divWithGrid.className = "grid";
            smartActionContainer.appendChild(divWithGrid);
        }

        const article = document.createElement("article");
        article.classList.add("hover-border");

        const articleHeader = document.createElement("header");
        articleHeader.id = `article-header-${key}`;
        const alias = document.createElement("h4");
        alias.innerHTML = obj?.alias || "";
        articleHeader.appendChild(alias);

        const description = document.createElement("div");
        description.innerHTML = obj?.description || "";

        const details = document.createElement("details");
        const summary = document.createElement("summary");
        summary.innerHTML = "Show details";
        const pre = document.createElement("pre");
        pre.innerHTML = JSON.stringify(obj.data, null, 2);
        details.appendChild(summary);
        details.appendChild(pre)

        const articleFooter = document.createElement("footer");
        articleFooter.id = `article_footer-${key}`;
        articleFooter.style.display = "flex";
        articleFooter.style.justifyContent = "flex-end";
        const deleteButton = document.createElement("div");
        deleteButton.textContent = "🗑️ ";
        deleteButton.id = `article_footer-${key}_delete_button`;
        const deleteButtonChangeListener = async () => {
            const store = await load('store.json', {autoSave: false});
            store.delete(key);
            await store.save();
            location.reload();
        };
        deleteButton.addEventListener("click", deleteButtonChangeListener);
        divEventListeners.push({
            elementId: deleteButton.id,
            elementInstance: deleteButton,
            listenerFn: deleteButtonChangeListener
        });

        articleFooter.appendChild(deleteButton);

        article.appendChild(articleHeader);
        article.appendChild(description);
        article.appendChild(document.createElement("br"));
        article.appendChild(details);
        article.appendChild(articleFooter);

        divWithGrid.appendChild(article);

        counterElementsInRow++;
    });
});