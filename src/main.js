const {invoke} = window.__TAURI__.core;
const {listen} = window.__TAURI__.event;

async function ui_notify_startup() {
    return await invoke("ui_notify_startup", {});
}

async function ui_notify_change_action(jsonSmartAction) {
    await invoke("ui_notify_change_action", {jsonSmartAction});
}

async function ui_request_execute_action(jsonSmartAction) {
    return await invoke("ui_request_execute_action", {jsonSmartAction});
}

async function ui_request_stop_action() {
    return await invoke("ui_request_stop_action", {});
}

let actions = [];
let currentSmartAction;
let inputListeners = [];

const form_action = document.getElementById("form_action");
const select_action = document.getElementById("select_action");
const input_ActionDescription = document.getElementById('input_action-description');
const div_actionProps = document.getElementById("div_action-props");
const button_submitFormAction = document.getElementById("button_submit-form-action");
// TODO: fix bug
const button_submitFormActionStopRecording = document.getElementById("button_submit-form-action-stop-recording");
const button_submitFormActionWait = document.getElementById("button_submit-form-action_wait");

function listen_smart_action_server_events() {
    listen('smart_action_recording_start', (event) => {
        console.log('Event received:', event.payload);

        // recording start
        button_submitFormAction.setAttribute("hidden", true);
        button_submitFormActionStopRecording.removeAttribute("hidden");
        button_submitFormActionWait.textContent = event.payload;
    });
    listen('smart_action_waiting_start', (event) => {
        console.log('Event received:', event.payload);

        // loading start
        button_submitFormAction.setAttribute("hidden", true);
        button_submitFormActionStopRecording.setAttribute("hidden", true);
        button_submitFormActionWait.removeAttribute("hidden");
        button_submitFormActionWait.textContent = event.payload;
    });
    listen('smart_action_waiting_stop', (event) => {
        console.log('Event received:', event.payload);

        // loading stop
        button_submitFormAction.removeAttribute("hidden");
        button_submitFormActionStopRecording.setAttribute("hidden", true);
        button_submitFormActionWait.setAttribute("hidden", true);
    });
    listen('smart_action_waiting_error', (event) => {
        console.log('Event received:', event.payload);

        // TODO: handle the error

        // loading stop
        button_submitFormAction.removeAttribute("hidden");
        button_submitFormActionStopRecording.setAttribute("hidden", true);
        button_submitFormActionWait.setAttribute("hidden", true);
    });
}

select_action.addEventListener('change', function () {
    populateViewForAction();
    notify_change_smart_action_to_tauri();
});

button_submitFormAction.addEventListener('click', async function (e) {
    e.preventDefault();

    let _result = await ui_request_execute_action(extractSmartActionJsonFromForm());
});

button_submitFormAction.addEventListener('click', async function (e) {
    e.preventDefault();

    let _result = await ui_request_stop_action();
});

window.addEventListener("DOMContentLoaded", async () => {
    listen_smart_action_server_events();

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

        select_action.selectedIndex = 0;
        populateViewForAction();
    }
});

function notify_change_smart_action_to_tauri() {
    ui_notify_change_action(extractSmartActionJsonFromForm());
}

function extractSmartActionJsonFromForm() {
    currentSmartAction = extractSmartActionFromForm();
    return JSON.stringify(currentSmartAction);
}

function extractSmartActionFromForm() {
    // This is gathered from the form fields
    const formData = new FormData(form_action);
    const data = Object.fromEntries(formData.entries());

    const selectedActionInUI = actions[data.value];

    const smartAction = {
        value: data.value,
        description: selectedActionInUI.description,
        name: selectedActionInUI.name,
        args: []
    };

    for (const [key, value] of Object.entries(data)) {
        if (key !== 'value' && key !== 'description') {
            let argument = selectedActionInUI.options[`${key}`];
            argument = argument.split("|")[0].trim();
            smartAction.args.push({
                [`${key}`]: value,
                arg: argument
            });
        }
    }

    return smartAction;
}

function populateViewForAction() {
    const actionValue = select_action.value;
    const action = actions[actionValue]
    input_ActionDescription.value = action.description;
    populateViewSettingsForAction(action);
}

function populateViewSettingsForAction(action) {
    console.log(`cleaning ${inputListeners?.length || 0} input listeners`);
    inputListeners.forEach(listener => {
        console.log("clean input listener for id: " + listener.elementId);
        listener.elementInstance.removeEventListener('input', listener.listenerFn);
    });
    inputListeners = [];
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
        const inputChangeListener = function (event) {
            console.log(event.target.value);
            notify_change_smart_action_to_tauri();
        }
        inputText.addEventListener('input', inputChangeListener);
        inputListeners.push({
            elementId: inputText.id,
            elementInstance: inputText,
            listenerFn: inputChangeListener
        });

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

    console.log(`attached ${inputListeners.length} input listeners`)
}

function convertSnakeToSpace(str) {
    return str.replace(/_/g, ' ');
}

function convertFirstCharToUppercase(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}