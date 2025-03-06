const {invoke} = window.__TAURI__.core;

let actions = [];

const selectAction = document.getElementById("select-action");
const selectedActionDescription = document.getElementById('selected-action-description');
const formActionProps = document.getElementById("form-action-props");

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
            console.log(`action key: ${action_key}, action props: ${action_props}`);

            const option = document.createElement('option');
            option.value = action_key;
            option.textContent = action_props.name;
            selectAction.appendChild(option);
        }

        selectAction.selectedIndex = 0;
        const action = actions[selectAction.value]
        selectedActionDescription.value = action.description;

        populateSettingsForAction(action);
    }

    selectAction.addEventListener('change', function () {
        const actionValue = selectAction.value;
        const action = actions[actionValue]
        selectedActionDescription.value = action.description;

        populateSettingsForAction(action);

        notify_change_action(actionValue, action.name)
    });
});

function populateSettingsForAction(action) {
    formActionProps.innerHTML = '';

    const maxElementsPerRow = 3;
    let counterElementPerRow = 0;
    let div;

    for (const [action_default_key, action_default_value] of Object.entries(action.defaults)) {
        console.log(`action key: ${action_default_key}`);
        console.log(`action val: ${action_default_value}`);

        if (counterElementPerRow % maxElementsPerRow === 0) {
            div = document.createElement("div");
            div.className = "grid";
            formActionProps.appendChild(div);
        }

        const inputText = document.createElement('input');
        inputText.type = 'text';
        inputText.value = action_default_value;
        inputText.id = 'form-action-props_input_' + action_default_key;

        const labelText = document.createElement('label');
        labelText.id = 'form-action-props_label_' + action_default_key
        labelText.htmlFor = inputText.id
        labelText.textContent = convertFirstCharToUppercase(convertSnakeToSpace(action_default_key));

        const innerDiv = document.createElement("div");
        innerDiv.appendChild(labelText);
        innerDiv.appendChild(inputText);
        div.appendChild(innerDiv);

        counterElementPerRow++;
    }
}

function convertSnakeToSpace(str) {
    return str.replace(/_/g, ' ');
}

function convertFirstCharToUppercase(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}