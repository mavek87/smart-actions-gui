const {invoke} = window.__TAURI__.core;

let actions = [];

const submitFormAction = document.getElementById("submit_form_action");
const formAction = document.getElementById("form-action");
const selectAction = document.getElementById("select-action");
const selectedActionDescription = document.getElementById('selected-action-description');
const divActionProps = document.getElementById("div-action-props");

async function notify_change_action(selectedActionValue, selectedActionName) {
    await invoke("notify_change_action", {value: selectedActionValue, name: selectedActionName});
    //alert(actionName);
}

async function notify_ui_startup() {
    return await invoke("notify_ui_startup", {});
}

window.addEventListener("DOMContentLoaded", async () => {
    const jsonOutput = await notify_ui_startup();

    if (jsonOutput) {
        const resultObj = JSON.parse(jsonOutput);

        actions = resultObj?.actions || [];

        for (const [action_key, action_props] of Object.entries(actions)) {

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

    formAction.addEventListener('submit', function (event) {
        event.preventDefault();

        const formData = new FormData(this);
        const data = Object.fromEntries(formData.entries());

        console.log(JSON.stringify(data));

        alert("data.action_value: " + data.action_value)
        alert("data.action_name: " + data.action_name)

        // TODO: ask server to exec action!  frontend.send(action) => server.exec(action)
    });
});

function populateSettingsForAction(action) {
    divActionProps.innerHTML = '';

    const maxElementsPerRow = 3;
    let counterElementsInRow = 0;
    let divWithGrid;

    for (const [action_default_key, action_default_value] of Object.entries(action.defaults)) {
        if (counterElementsInRow % maxElementsPerRow === 0) {
            divWithGrid = document.createElement("div");
            divWithGrid.className = "grid";
            divActionProps.appendChild(divWithGrid);
        }

        const inputText = document.createElement('input');
        inputText.type = 'text';
        inputText.value = action_default_value || "";
        inputText.id = 'form-action-props_input_' + action_default_key;
        inputText.name = action_default_key;

        const labelText = document.createElement('label');
        labelText.id = 'form-action-props_label_' + action_default_key
        labelText.htmlFor = inputText.id
        labelText.textContent = convertFirstCharToUppercase(convertSnakeToSpace(action_default_key));

        const innerDiv = document.createElement("div");
        innerDiv.appendChild(labelText);
        innerDiv.appendChild(inputText);

        divWithGrid.appendChild(innerDiv);

        counterElementsInRow++;
    }
}

function convertSnakeToSpace(str) {
    return str.replace(/_/g, ' ');
}

function convertFirstCharToUppercase(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}