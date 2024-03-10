use open_ai_client::{
    AudioModel, AudioResponse, AudioResponseFormat, CreateSpeechRequest,
    CreateTranscriptionRequest, CreateTranslationRequest, OpenAIClient, OpenAIFile,
    SpeechResponseFormat, TtsModel, Voice,
};

pub async fn audio_create_speech_test(client: OpenAIClient) {
    let request = CreateSpeechRequest {
        model: TtsModel::Tts1,
        input: "I'm saying hello world in rust".to_string(),
        voice: Voice::Echo,
        response_format: SpeechResponseFormat::Mp3,
        speed: Some(1.0),
    };

    let result = client.audio().create_speech(request).await.unwrap();

    assert!(!result.0.is_empty());
}

pub async fn audio_transcriptions_test(client: OpenAIClient) {
    let request = CreateTranscriptionRequest {
        file: OpenAIFile::from_path("tests/resources/audio/one_small_step_for_man.mp3")
            .await
            .unwrap(),
        model: AudioModel::Whisper1,
        prompt: None,
        response_format: AudioResponseFormat::Json,
        temperature: None,
        language: Some("en".to_string()),
    };

    let result = client.audio().create_transcription(request).await.unwrap();

    let expected = AudioResponse {
        text: "I'm at the foot of the ladder. The lamb foot pads are only depressed in the surface about one or two inches, \
        although the surface appears to be very, very fine-grained as you get close to it. \
        It's almost like a powder. Ground mass is very fine. I'm going to step off the lamb now. \
        That's one small step for man, one giant leap for mankind.".to_string(),
    };

    assert_eq!(result, expected);
}

pub async fn audio_translations_test(client: OpenAIClient) {
    let request = CreateTranslationRequest {
        file: OpenAIFile::from_path("tests/resources/audio/french_sample.mp3")
            .await
            .unwrap(),
        model: AudioModel::Whisper1,
        prompt: None,
        response_format: AudioResponseFormat::Json,
        temperature: None,
    };

    let result = client.audio().create_translations(request).await.unwrap();

    let expected = AudioResponse {
        text: "The first time you open a Customer Interaction, you will be directed to the Identification page. \
        This is the default mode used for all Customer Interactions. \
        Please check the caller's Social Security number before proceeding. Once you have confirmed, \
        click the following button, like this. Very good. Now let's move on to step 2.".to_string(),
    };

    assert_eq!(result, expected);
}
