export function buildMetadataIfIsSelect(action_key) {
    switch (action_key) {
        case "audio_sampling_rate":
            const audioSamplingRate = {
                "tooltip": "This is the sampling rate of the recording",
                "values": [
                    {"value": "44100", "name": "44100"},
                    {"value": "48000", "name": "48000"},
                ]
            }
            return audioSamplingRate;
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
        case "output_format":
            const outputFormatOptionsMetadata = {
                "tooltip": "The output format can be text format (multiple lines) or string format (one line)",
                "values": [
                    {"value": "string", "name": "String"},
                    {"value": "text", "name": "Text"},
                ]
            }

            if (select_action.value === "ai_reply_text") {
                outputFormatOptionsMetadata.values.push(
                    {"value": "code_string", "name": "Code String"},
                    {"value": "code_text", "name": "Code Text"},
                )
            }

            return outputFormatOptionsMetadata;
        case "output_terminator":
            const outputTerminatorOptionsMetadata = {
                // NOTE: probably text is a better default here for a GUI instead of string which is better for the CLI software
                "defaultValue": "none",
                "tooltip": "The output of the smart action can end with a Enter character or nothing more than the output itself",
                "values": [
                    {"value": "none", "name": "None"},
                    {"value": "enter", "name": "Enter"},
                ]
            }
            return outputTerminatorOptionsMetadata;
        default:
            return null;
    }
}