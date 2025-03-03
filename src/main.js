const {invoke} = window.__TAURI__.core;

let actions = [];

async function notify_change_action(selectedActionName) {
    await invoke("notify_change_action", {name: selectedActionName});
    //alert(actionName);
}

window.addEventListener("DOMContentLoaded", () => {
    const selectAction = document.getElementById("select-action");
    const selectedActionDescription = document.getElementById('selected-action-description');

    fetch('assets/actions.json')
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            data.actions.forEach((item, index) => {
                let action = {
                    index: index,
                    value: item.value,
                    name: item.name,
                    description: item.description
                }

                actions.push(action);

                const option = document.createElement('option');
                option.value = item.value;
                option.textContent = item.name;
                selectAction.appendChild(option);
            });

            selectAction.selectedIndex = 0;
            selectedActionDescription.value = actions.at(0).description;
        })
        .catch(error => {
            window.alert('There was a problem with the fetch operation: ' + error);
        });

    selectAction.addEventListener('change', function () {
        const actionIndex = selectAction.selectedIndex;
        const action = actions.at(actionIndex);
        selectedActionDescription.value = action.description;
        selectedActionDescription.tooltip = action.description;
        notify_change_action(action.value)
    });
});
