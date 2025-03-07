const {invoke} = window.__TAURI__.core;

let actions = [];

const submitFormAction = document.getElementById("submit_form_action");
const formAction = document.getElementById("form-action");
const selectAction = document.getElementById("select-action");
const selectedActionDescription = document.getElementById('selected-action-description');
const divActionProps = document.getElementById("div-action-props");

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

window.addEventListener("DOMContentLoaded", async () => {
    const jsonOutput = await ui_notify_startup();

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

        ui_notify_change_action(actionValue, action.name)
    });

    // TODO: a code refactoring is needed
    formAction.addEventListener('submit', function (event) {
        event.preventDefault();

        // This is gathered from the form fields
        const formData = new FormData(this);
        const data = Object.fromEntries(formData.entries());

        const selectedActionInUI = actions[selectAction.value];
        // console.log(selectedActionInUI);
        let selectedAction = selectAction.options[selectAction.selectedIndex];
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

        const jsonAction = JSON.stringify(action);

        ui_request_execute_action(jsonAction);
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