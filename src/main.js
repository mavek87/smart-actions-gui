const {invoke} = window.__TAURI__.core;

let actions = [];

const selectAction = document.getElementById("select-action");
const selectedActionDescription = document.getElementById('selected-action-description');

async function notify_change_action(selectedActionValue, selectedActionName) {
    await invoke("notify_change_action", {value: selectedActionValue, name: selectedActionName});
    //alert(actionName);
}

async function notify_ui_startup() {
    return await invoke("notify_ui_startup", {});
}

window.addEventListener("DOMContentLoaded", async () => {
    const jsonOutput = await notify_ui_startup();
    console.log(jsonOutput);

    if (jsonOutput) {
        const resultObj = JSON.parse(jsonOutput);
        console.log(resultObj);

        actions = resultObj?.actions || [];

        for (const [action_key, action_props] of Object.entries(actions)) {
            console.log(`key: ${action_key}, value: ${action_props}`);

            const option = document.createElement('option');
            option.value = action_key;
            option.textContent = action_props.name;
            selectAction.appendChild(option);
        }

        selectAction.selectedIndex = 0;
        const action = actions[selectAction.value]
        selectedActionDescription.value = action.description;
    }

    selectAction.addEventListener('change', function () {
        const actionValue = selectAction.value;
        const action = actions[actionValue]
        selectedActionDescription.value = action.description;

        notify_change_action(actionValue, action.name)
    });
});