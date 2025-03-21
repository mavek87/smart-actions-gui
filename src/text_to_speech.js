export function buildMetadataIfIsSelect(action_key, select_action, is_audio_enabled) {
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
            return languageOptionsMetadata;
        default:
            return null
    }
}

export function buildTextToSpeechMetadataIfIsTextArea(action_key, select_action, is_audio_enabled) {
    switch (action_key) {
        case "text":
            const textOptionsMetadata =
                {
                    // "defaultValue": "",
                    // "tooltip": "Suggest a language to use by the speech to text software, otherwise it will find out what language is spoken by the user",
                    // "values": [
                    //     {"value": "", "name": "None"},
                    //     {"value": "en", "name": "English"},
                    //     {"value": "it", "name": "Italian"},
                    //     {"value": "es", "name": "Spanish"},
                    //     {"value": "fr", "name": "French"},
                    // ]
                }
            return textOptionsMetadata;
        default:
            return null
    }
}