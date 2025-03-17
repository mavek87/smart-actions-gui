export function buildMetadataIfIsSelect(action_key) {
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
            return modelOptionsMetadata;
        case "task":
            const taskOptionsMetadata = {
                "defaultValue": "transcribe",
                "tooltip": "The speech to text model can transcribe what it hears or translate it into english",
                "values": [
                    {"value": "transcribe", "name": "Transcribe"},
                    {"value": "translate", "name": "Translate"},
                ]
            }
            return taskOptionsMetadata;
        default:
            return null;
    }
}