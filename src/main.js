const {invoke} = window.__TAURI__.core;
const {listen} = window.__TAURI__.event;

import {buildMetadataIfIsSelect as buildDictateTextMetadataIfIsSelect} from './dictate_text.js';
import {buildMetadataIfIsSelect as buildAiReplyTextMetadataIfIsSelect} from './ai_reply_text.js';
import {buildMetadataIfIsSelect as buildAudioToTextMetadataIfIsSelect} from './audio_to_text.js';
import {buildMetadataIfIsSelect as buildTextToSpeechMetadataIfIsSelect} from './text_to_speech.js';

async function ui_notify_startup() {
    return await invoke("ui_notify_startup", {});
}

async function ui_notify_change_action(jsonSmartAction) {
    await invoke("ui_notify_change_action", {jsonSmartAction});
}

async function ui_notify_change_element_in_action(jsonSmartAction) {
    await invoke("ui_notify_change_element_in_action", {jsonSmartAction});
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
let is_audio_enabled = true;

const form_action = document.getElementById("form_action");
const select_action = document.getElementById("select_action");
const input_ActionDescription = document.getElementById('input_action-description');
const div_actionProps = document.getElementById("div_action-props");
const button_submitFormAction = document.getElementById("button_submit-form-action");
const button_submitFormActionStopRecording = document.getElementById("button_submit-form-action-stop-recording");
const button_submitFormActionWait = document.getElementById("button_submit-form-action_wait");

select_action.addEventListener('change', function () {
    populateViewForAction();
    ui_notify_change_action(extractSmartActionJsonFromForm())
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
        listen('event_to_ui_next_smart_action', event => {
            console.log('request_to_ui_next_smart_action - Event received:', event.payload);
            selectNextAction();
        });
        listen('event_to_ui_previous_smart_action', event => {
            console.log('request_to_ui_previous_smart_action - Event received:', event.payload);
            selectPreviousAction();
        })
        listen('event_to_ui_smart_action_recording_start', (event) => {
            console.log('Event received:', event.payload);

            button_submitFormAction.setAttribute("hidden", true);
            button_submitFormActionStopRecording.removeAttribute("hidden");
            button_submitFormActionWait.textContent = event.payload;
        });
        listen('event_to_ui_smart_action_waiting_start', (event) => {
            console.log('Event received:', event.payload);

            button_submitFormAction.setAttribute("hidden", true);
            button_submitFormActionStopRecording.setAttribute("hidden", true);
            button_submitFormActionWait.removeAttribute("hidden");
            button_submitFormActionWait.textContent = event.payload;
        });
        listen('event_to_ui_smart_action_waiting_stop', (event) => {
            console.log('Event received:', event.payload);

            button_submitFormAction.removeAttribute("hidden");
            button_submitFormActionStopRecording.setAttribute("hidden", true);
            button_submitFormActionWait.setAttribute("hidden", true);
        });
        listen('event_to_ui_smart_action_waiting_error', (event) => {
            console.log('Event received:', event.payload);

            // TODO: handle the error

            button_submitFormAction.removeAttribute("hidden");
            button_submitFormActionStopRecording.setAttribute("hidden", true);
            button_submitFormActionWait.setAttribute("hidden", true);
        });
        listen('event_to_ui_enable_audio_changed', (event) => {
            console.log('event_to_ui_enable_audio_changed received:', event.payload);
            is_audio_enabled = event.payload;
            let select_output_audio_voice = document.getElementById("form-action-props_select_output_audio_voice");
            if (select_output_audio_voice) {
                select_output_audio_voice.disabled = !is_audio_enabled;
                if (!is_audio_enabled) {
                    select_output_audio_voice.value = "false";
                }
            }
        });
    }

    listen_smart_action_server_events();

    const jsonStartupData = await ui_notify_startup();

    if (jsonStartupData) {
        const startupDataObject = JSON.parse(jsonStartupData);

        actions = startupDataObject?.actions || [];
        is_audio_enabled = startupDataObject?.is_audio_enabled;

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
            if (!(key === 'output_terminator' && value === 'None')) {
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
    let actionValue = select_action.value;
    const action = actions[actionValue]
    input_ActionDescription.value = action.description;
    action.value = actionValue;
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

        const element = buildElementForActionType(action, action_default_key, action_default_value);

        divWithGrid.appendChild(element);

        counterElementsInRow++;
    }

    console.log(`attached ${inputListeners.length} input listeners`)
}

function buildElementForActionType(action, action_key, action_value) {
    console.log(JSON.stringify(action))
    let selectMetadata;
    switch (action.value) {
        case "dictate_text":
            selectMetadata = buildDictateTextMetadataIfIsSelect(action_key);
            if (selectMetadata) {
                return buildSelectElement(action_key, action_value, selectMetadata);
            }
            break;
        case "audio_to_text":
            selectMetadata = buildAudioToTextMetadataIfIsSelect(action_key);
            if (selectMetadata) {
                return buildSelectElement(action_key, action_value, selectMetadata);
            }
            break;
        case "ai_reply_text":
            selectMetadata = buildAiReplyTextMetadataIfIsSelect(action_key, select_action, is_audio_enabled);
            if (selectMetadata) {
                return buildSelectElement(action_key, action_value, selectMetadata);
            }
            break;
        case "text_to_speech":
            selectMetadata = buildTextToSpeechMetadataIfIsSelect(action_key);
            if (selectMetadata) {
                return buildSelectElement(action_key, action_value, selectMetadata);
            }
            break;
        default:
            console.log(`Unknown action: ${action_key}`);
            break;
    }
    return buildDefaultElement(action_key, action_value);
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
        ui_notify_change_element_in_action(extractSmartActionJsonFromForm());
    }
    select.addEventListener('change', selectChangeListener);
    inputListeners.push({
        elementId: select.id,
        elementInstance: select,
        listenerFn: selectChangeListener
    });
    if (action_key === "output_audio_voice" && !is_audio_enabled) {
        select.disabled = true;
    }

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

function buildDefaultElement(action_key, action_value) {
    const inputText = document.createElement('input');
    inputText.type = 'text';
    inputText.value = action_value || "";
    inputText.id = 'form-action-props_input_' + action_key;
    inputText.name = action_key;
    const inputChangeListener = function (event) {
        console.log(event.target.value);
        ui_notify_change_element_in_action(extractSmartActionJsonFromForm());
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

function convertSnakeToSpace(str) {
    return str.replace(/_/g, ' ');
}

function convertFirstCharToUppercase(str) {
    return str.charAt(0).toUpperCase() + str.slice(1);
}