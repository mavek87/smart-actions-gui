const {invoke} = window.__TAURI__.core;

async function ui_notify_change_action(selectedActionValue, selectedActionName) {
    await invoke("ui_notify_change_action", {value: selectedActionValue, name: selectedActionName});
    //alert(actionName);
}

async function ui_notify_startup() {
    return await invoke("ui_notify_startup", {});
}

async function ui_request_execute_action(jsonAction) {
    await invoke("ui_request_execute_action", {jsonAction});
}

// This are input actions
let actions = [];
// This is an output action
let currentAction;
// Their json structure is not the same!!!

const button_submitFormAction = document.getElementById("button_submit-form-action");
const form_action = document.getElementById("form_action");
const select_action = document.getElementById("select_action");
const input_ActionDescription = document.getElementById('input_action-description');
const div_actionProps = document.getElementById("div_action-props");

button_submitFormAction.addEventListener('click', function (e) {
    e.preventDefault();

    currentAction = extractCurrentActionFromForm();

    const jsonAction = JSON.stringify(currentAction);

    ui_request_execute_action(jsonAction);
});

window.addEventListener("DOMContentLoaded", async () => {
    const jsonActions = await ui_notify_startup();

    if (jsonActions) {
        const actionsWrapperObj = JSON.parse(jsonActions);

        actions = actionsWrapperObj?.actions || [];

        for (const [action_key, action_props] of Object.entries(actions)) {
            const option = document.createElement('option');
            option.value = action_key;
            option.textContent = action_props.name;
            select_action.appendChild(option);
        }

        // TODO: fix needed. When started action is not aligned with tray bar
        select_action.selectedIndex = 0;
        const action = actions[select_action.value]
        input_ActionDescription.value = action.description;

        populateSettingsForAction(action);
    }

    select_action.addEventListener('change', function () {
        const actionValue = select_action.value;
        const action = actions[actionValue]
        input_ActionDescription.value = action.description;

        populateSettingsForAction(action);

        ui_notify_change_action(actionValue, action.name)
    });
});

function extractCurrentActionFromForm() {
    // This is gathered from the form fields
    const formData = new FormData(form_action);
    const data = Object.fromEntries(formData.entries());

    const selectedActionInUI = actions[select_action.value];
    // console.log(selectedActionInUI);
    let selectedAction = select_action.options[select_action.selectedIndex];
    let selectedActionName = selectedAction.text;

    const action = {
        value: data.value,
        name: selectedActionName,
        description: data.description,
        args: []
    };

    for (const [key, value] of Object.entries(data)) {
        if (key !== 'value' && key !== 'description') {
            let argument = selectedActionInUI.options[`${key}`];
            argument = argument.split("|")[0].trim();
            action.args.push({
                [`${key}`]: value,
                arg: argument
            });
        }
    }

    return action;
}

function populateSettingsForAction(action) {
    div_actionProps.innerHTML = '';

    const maxElementsPerRow = 3;
    let counterElementsInRow = 0;
    let divWithGrid;

    for (const [action_default_key, action_default_value] of Object.entries(action.defaults)) {
        if (counterElementsInRow % maxElementsPerRow === 0) {
            divWithGrid = document.createElement("div");
            divWithGrid.className = "grid";
            div_actionProps.appendChild(divWithGrid);
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