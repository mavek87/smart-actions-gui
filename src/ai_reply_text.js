export function buildMetadataIfIsSelect(action_key, select_action, is_audio_enabled) {
    switch (action_key) {
        case "audio_sampling_rate":
            return buildSelectMetadata({
                "tooltip": "This is the sampling rate of the recording",
                "values": [
                    {"value": "44100", "name": "44100"},
                    {"value": "48000", "name": "48000"},
                ]
            });
        case "ai_provider":
            return buildSelectMetadata({
                "tooltip": "This is the AI provider that will be used to generate the response",
                "values": [
                    {"value": "openai", "name": "OpenAI"},
                    {"value": "pollinations", "name": "Pollinations"},
                    {"value": "duckduckgo", "name": "DuckDuckGo"},
                    {"value": "ollama", "name": "Ollama"},
                    {"value": "phind", "name": "Phind"},
                ]
            });
        case "language":
            return buildSelectMetadata({
                "defaultValue": "",
                "tooltip": "Suggest a language to use by the speech to text software, otherwise it will find out what language is spoken by the user",
                "values": [
                    {"value": "", "name": "None"},
                    {"value": "en", "name": "English"},
                    {"value": "it", "name": "Italian"},
                    {"value": "es", "name": "Spanish"},
                    {"value": "fr", "name": "French"},
                ]
            });
        case "selection_target":
            return buildSelectMetadata({
                "defaultValue": "none",
                "tooltip": "If 'none' isn't selected the AI can take into account the selected text or the text copied into the clipboard",
                // NOTE: terminal doesn't make sense in a GUI, so it's omitted
                "values": [
                    {"value": "none", "name": "None"},
                    {"value": "primary", "name": "Selected Text"},
                    {"value": "clipboard", "name": "Copied Text"},
                ]
            });
        case "output_destination":
            return buildSelectMetadata({
                "defaultValue": "display",
                // NOTE: terminal doesn't make sense in a GUI, so it's omitted
                "values": [
                    {"value": "display", "name": "Display"},
                ]
            });
        case "model":
            return buildSelectMetadata({
                "defaultValue": "None",
                "tooltip": "The model used by the speach to text software (higher = more accurate, lower = faster)",
                "values": [
                    {"value": "", "name": "None"},
                    {"value": "llama3:latest", "name": "llama3:latest"},
                    {"value": "small", "name": "Small"},
                    {"value": "medium", "name": "Medium"},
                    {"value": "large", "name": "Large"},
                ]
            });
        case "url":
            return buildSelectMetadata({
                "defaultValue": "None",
                "tooltip": "The model used by the speach to text software (higher = more accurate, lower = faster)",
                "values": [
                    {"value": "", "name": "None"},
                    {"value": "http://localhost:11434/v1/chat/completions", "name": "http://localhost:11434/v1/chat/completions"},
                ]
            });
        case "task":
            return buildSelectMetadata({
                "defaultValue": "transcribe",
                "tooltip": "The speech to text model can transcribe what it hears or translate it into english",
                "values": [
                    {"value": "transcribe", "name": "Transcribe"},
                    {"value": "translate", "name": "Translate"},
                ]
            });
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
        case "output_audio_voice":
            const outputAudioVoice = {
                "tooltip": "If it's true the output text will also be read by a speech to text software, otherwise if false this doesn't happen",
                "values": [
                    {"value": "false", "name": "false"},
                    {"value": "true", "name": "true"},
                ]
            }

            outputAudioVoice.defaultValue = `${is_audio_enabled}`;

            return outputAudioVoice;
        default:
            return null
    }
}

function buildSelectMetadata({tooltip, values, defaultValue}) {
    const metadata = {tooltip, values};
    if (defaultValue !== undefined) {
        metadata.defaultValue = defaultValue;
    }
    return metadata;
}