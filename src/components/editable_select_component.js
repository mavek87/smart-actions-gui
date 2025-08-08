// USAGE EXAMPLE:
//
// import { createEditableSelect } from './editableSelect.js'
//
// const options = [
//     { label: 'Apple', value: 'apple' },
//     { label: 'Banana', value: 'banana' },
//     { label: 'Cherry', value: 'cherry' },
// ]
//
// const editableSelect = createEditableSelect(options, 'Select a fruit')
//
// document.body.appendChild(editableSelect.element)
//
// // Prendere il valore selezionato o aggiunto
// console.log('Selected:', editableSelect.getValue())
//
// // Impostare un valore
// editableSelect.setValue('banana')
//

export function createEditableSelect(options, placeholder = '', newChoiceLabel = 'Add new choice') {
    const container = document.createElement('div')

    const sel = document.createElement('select')
    const defaultOpt = new Option(placeholder, '')
    sel.appendChild(defaultOpt)
    sel.appendChild(new Option('Add new choice...', newChoiceLabel))
    options.forEach(opt => sel.appendChild(new Option(opt.label, opt.value)))

    const inputContainer = document.createElement('div')
    inputContainer.style.display = 'none'
    inputContainer.style.alignItems = 'baseline'
    inputContainer.style.gap = '0.5em'

    const input = document.createElement('input')
    input.placeholder = 'Enter the new value'

    const saveBtn = document.createElement('button')
    saveBtn.textContent = 'Save'

    const cancelBtn = document.createElement('button')
    cancelBtn.textContent = 'Cancel'

    inputContainer.append(input, saveBtn, cancelBtn)
    container.append(sel, inputContainer)

    let previousValue = ''  // **aggiunto** per tenere traccia del valore precedente

    function reset() {
        inputContainer.style.display = 'none'
        sel.style.display = 'inline-block'
        sel.value = previousValue  // **ripristino valore precedente**
    }

    function saveValue() {
        const val = input.value.trim()
        if (!val) return alert('Empty value!')
        if ([...sel.options].some(o => o.value === val)) return alert('Value already exists!')
        const opt = new Option(val, val, true, true)
        sel.add(opt)
        sel.value = val
        previousValue = val      // **aggiorno previousValue con nuovo valore**
        reset()
    }

    sel.onchange = () => {
        if (sel.value === newChoiceLabel) {
            if (previousValue === newChoiceLabel) previousValue = ''
            sel.style.display = 'none'
            inputContainer.style.display = 'flex'
            input.value = ''
            input.focus()
        } else {
            previousValue = sel.value
        }
    }

    saveBtn.onclick = e => {
        e.preventDefault()
        saveValue()
    }

    cancelBtn.onclick = e => {
        e.preventDefault()
        reset()
    }

    input.onkeydown = e => {
        if (e.key === 'Enter') {
            e.preventDefault()
            saveValue()
        } else if (e.key === 'Escape') {
            e.preventDefault()
            reset()
        }
    }

    return {
        element: container,
        getSelect: () => sel,
        getValue: () => sel.value,
        setValue: val => {
            sel.value = val
            previousValue = val  // **aggiorno anche qui previousValue**
        },
    }
}