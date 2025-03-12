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
const button_submitFormActionStopRecording = document.getElementById("button_submit-form-action-stop-recording");
const button_submitFormActionWait = document.getElementById("button_submit-form-action_wait");

select_action.addEventListener('change', function () {
    populateViewForAction();
    notify_change_smart_action_to_tauri();
});

button_submitFormAction.addEventListener('click', async function (e) {
    e.preventDefault();

    let _result = await ui_request_execute_action(extractSmartActionJsonFromForm());
});

button_submitFormActionStopRecording.addEventListener('click', async function (e) {
    e.preventDefault();

    let _result = await ui_request_stop_action();
});

window.addEventListener("DOMContentLoaded", async () => {
    function listen_smart_action_server_events() {
        listen('request_to_ui_next_smart_action', event => {
            console.log('request_to_ui_next_smart_action - Event received:', event.payload);
            selectNextAction();
        });
        listen('request_to_ui_previous_smart_action', event => {
            console.log('request_to_ui_previous_smart_action - Event received:', event.payload);
            selectPreviousAction();
        })
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

    select_action.title = "Select a smart action to execute";
});

function notify_change_smart_action_to_tauri() {
    ui_notify_change_action(extractSmartActionJsonFromForm());
}

function selectNextAction() {
    const optionsCount = select_action.options.length;
    const selectedIndex = select_action.selectedIndex;
    if (selectedIndex < (optionsCount - 1)) {
        select_action.selectedIndex = selectedIndex + 1;
    } else {
        select_action.selectedIndex = 0;
    }
    select_action.dispatchEvent(new Event('change'));
}

function selectPreviousAction() {
    const optionsCount = select_action.options.length;
    const selectedIndex = select_action.selectedIndex;
    if (selectedIndex > 0) {
        select_action.selectedIndex = selectedIndex - 1;
    } else {
        select_action.selectedIndex = optionsCount - 1;
    }
    select_action.dispatchEvent(new Event('change'));
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
            if (!(key === 'output_terminator' && value === 'none')) {
                let argument = selectedActionInUI.options[`${key}`];
                argument = argument.split("|")[0].trim();
                smartAction.args.push({
                    [`${key}`]: value,
                    arg: argument
                });
            }
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

        const element = buildElementForActionType(action_default_key, action_default_value);

        divWithGrid.appendChild(element);

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

function buildElementForActionType(action_key, action_value) {
    switch (action_key) {
        case "language":
            const languageOptionsMetadata = {
                "defaultValue": "",
                "tooltip": "Suggest a language to use by the speech to text software, otherwise it will find out what language is spoken by the user",
                "values": [
                    {"value": "", "name": "None"},
                    {"value": "en", "name": "English"},
                    {"value": "it", "name": "Italian"},
                    {"value": "es", "name": "Spanish"},
                    {"value": "fr", "name": "French"},
                ]
            }
            return buildSelectElement(action_key, action_value, languageOptionsMetadata);
        case "selection_target":
            const selectionTargetOptionsMetadata = {
                // NOTE: terminal doesn't make sense in a GUI, so it's omitted
                "defaultValue": "none",
                "tooltip": "If 'none' isn't selected the AI can take into account the selected text or the text copied into the clipboard",
                "values": [
                    {"value": "none", "name": "None"},
                    {"value": "primary", "name": "Selected Text"},
                    {"value": "clipboard", "name": "Copied Text"},
                ]
            }
            return buildSelectElement(action_key, action_value, selectionTargetOptionsMetadata);
        case "output_destination":
            const outputDestinationOptionsMetadata = {
                // NOTE: terminal doesn't make sense in a GUI, so it's omitted
                "defaultValue": "display",
                "values": [
                    {"value": "display", "name": "Display"},
                ]
            }
            return buildSelectElement(action_key, action_value, outputDestinationOptionsMetadata);
        case "model":
            const modelOptionsMetadata = {
                "defaultValue": "medium",
                "tooltip": "The model used by the speach to text software (higher = more accurate, lower = faster)",
                "values": [
                    {"value": "small", "name": "Small"},
                    {"value": "medium", "name": "Medium"},
                    {"value": "large", "name": "Large"},
                ]
            }
            return buildSelectElement(action_key, action_value, modelOptionsMetadata);
        case "task":
            const taskOptionsMetadata = {
                "defaultValue": "transcribe",
                "tooltip": "The speech to text model can transcribe what it hears or translate it into english",
                "values": [
                    {"value": "transcribe", "name": "Transcribe"},
                    {"value": "translate", "name": "Translate"},
                ]
            }
            return buildSelectElement(action_key, action_value, taskOptionsMetadata);
        case "output_format":
            const outputFormatOptionsMetadata = {
                "tooltip": "The output format can be text format (multiple lines) or string format (one line)",
                "values": [
                    {"value": "string", "name": "String"},
                    {"value": "text", "name": "Text"},
                ]
            }
            return buildSelectElement(action_key, action_value, outputFormatOptionsMetadata);
        case "output_terminator":
            const outputTerminatorOptionsMetadata = {
                // NOTE: probably text is a better default here for a GUI instead of string which is better for the CLI software
                "defaultValue": "None",
                "tooltip": "The output of the smart action can end with a Enter character or nothing more than the output itself",
                "values": [
                    {"value": "None", "name": "None"},
                    {"value": "Enter", "name": "Enter"},
                ]
            }
            return buildSelectElement(action_key, action_value, outputTerminatorOptionsMetadata);
        default:
            return buildDefaultElement(action_key, action_value);
    }
}

function buildDefaultElement(action_key, action_value) {
    const inputText = document.createElement('input');
    inputText.type = 'text';
    inputText.value = action_value || "";
    inputText.id = 'form-action-props_input_' + action_key;
    inputText.name = action_key;
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
    labelText.id = 'form-action-props_label_' + action_key
    labelText.htmlFor = inputText.id
    labelText.textContent = convertFirstCharToUppercase(convertSnakeToSpace(action_key));

    const innerDiv = document.createElement("div");
    innerDiv.appendChild(labelText);
    innerDiv.appendChild(inputText);
    return innerDiv;
}

function buildSelectElement(action_key, action_value, optionsMetadata) {
    const innerDiv = document.createElement("div");

    const select = document.createElement("select");
    optionsMetadata.values.forEach(optionMetadata => {
        const uiOption = document.createElement("option");
        uiOption.value = optionMetadata.value;
        uiOption.textContent = optionMetadata.name;
        select.appendChild(uiOption);
    });

    select.value = optionsMetadata.defaultValue || action_value;
    select.id = 'form-action-props_select_' + action_key;
    select.name = action_key;
    const selectChangeListener = function (event) {
        console.log(event.target.value);
        notify_change_smart_action_to_tauri();
    }
    select.addEventListener('change', selectChangeListener);
    inputListeners.push({
        elementId: select.id,
        elementInstance: select,
        listenerFn: selectChangeListener
    });

    if (optionsMetadata.tooltip) {
        select.title = optionsMetadata.tooltip;
    }

    const labelText = document.createElement('label');
    labelText.id = 'form-action-props_label_' + action_key
    labelText.htmlFor = select.id
    labelText.textContent = convertFirstCharToUppercase(convertSnakeToSpace(action_key));

    innerDiv.appendChild(labelText);
    innerDiv.appendChild(select);
    return innerDiv;
}