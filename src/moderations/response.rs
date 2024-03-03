use serde::{Deserialize, Serialize};

/// Represents policy compliance report by OpenAI's content moderation model against a given input.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateResponse {
    /// The unique identifier for the moderation request.
    pub id: String,

    /// The model used to generate the moderation results.
    pub model: String,

    /// A list of moderation objects.
    pub results: Vec<ModerationObject>,
}

/// Moderation object
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModerationObject {
    /// Whether the content violates [OpenAI's usage policies](https://platform.openai.com/policies/usage-policies).
    pub flagged: bool,

    /// A list of the categories, and whether they are flagged or not.
    pub categories: Categories,

    /// A list of the categories along with their scores as predicted by model.
    pub category_scores: CategoryScores,
}

/// A category of moderated content
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Categories {
    /// Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status,
    /// or caste. Hateful content aimed at non-protected groups (e.g., chess players) is harrassment.
    pub hate: bool,

    /// Hateful content that also includes violence or serious harm towards the targeted group based on race, gender, ethnicity, religion, nationality,
    /// sexual orientation, disability status, or caste.
    #[serde(alias = "hate/threatening")]
    pub hate_threatening: bool,

    /// Content that expresses, incites, or promotes harassing language towards any target.
    pub harassment: bool,

    /// Harassment content that also includes violence or serious harm towards any target.
    #[serde(alias = "harassment/threatening")]
    pub harassment_threatening: bool,

    /// Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(alias = "self-harm")]
    pub self_harm: bool,

    /// Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(alias = "self-harm/intent")]
    pub self_harm_intent: bool,

    /// Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders,
    /// or that gives instructions or advice on how to commit such acts.
    #[serde(alias = "self-harm/instructions")]
    pub self_harm_instructions: bool,

    /// Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness).
    pub sexual: bool,

    /// Sexual content that includes an individual who is under 18 years old.
    #[serde(alias = "sexual/minors")]
    pub sexual_minors: bool,

    /// Content that depicts death, violence, or physical injury.
    pub violence: bool,

    /// Content that depicts death, violence, or physical injury in graphic detail.
    #[serde(alias = "violence/graphic")]
    pub violence_graphic: bool,
}

/// Moderated content category scores
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CategoryScores {
    ///The score for the category 'hate'.
    pub hate: f32,

    /// The score for the category 'hate/threatening'.
    #[serde(alias = "hate/threatening")]
    pub hate_threatening: f32,

    /// The score for the category 'harassment'.
    pub harassment: f32,

    /// The score for the category 'harassment/threatening'.
    #[serde(alias = "harassment/threatening")]
    pub harassment_threatening: f32,

    /// The score for the category 'self-harm'.
    #[serde(alias = "self-harm")]
    pub self_harm: f32,

    /// The score for the category 'self-harm/intent'.
    #[serde(alias = "self-harm/intent")]
    pub self_harm_intent: f32,

    /// The score for the category 'self-harm/instructions'.
    #[serde(alias = "self-harm/instructions")]
    pub self_harm_instructions: f32,

    /// The score for the category 'sexual'.
    pub sexual: f32,

    /// The score for the category 'sexual/minors'.
    #[serde(alias = "sexual/minors")]
    pub sexual_minors: f32,

    /// The score for the category 'violence'.
    pub violence: f32,

    /// The score for the category 'violence/graphic'.
    #[serde(alias = "violence/graphic")]
    pub violence_graphic: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn deserializes_response_correctly() {
        let json = json!({
          "id": "modr-XXXXX",
          "model": "text-moderation-005",
          "results": [
            {
              "flagged": true,
              "categories": {
                "sexual": false,
                "hate": false,
                "harassment": false,
                "self-harm": false,
                "sexual/minors": false,
                "hate/threatening": false,
                "violence/graphic": false,
                "self-harm/intent": false,
                "self-harm/instructions": false,
                "harassment/threatening": true,
                "violence": true,
              },
              "category_scores": {
                "sexual": 1.2282071e-06,
                "hate": 0.010696256,
                "harassment": 0.29842457,
                "self-harm": 1.5236925e-08,
                "sexual/minors": 5.7246268e-08,
                "hate/threatening": 0.0060676364,
                "violence/graphic": 4.435014e-06,
                "self-harm/intent": 8.098441e-10,
                "self-harm/instructions": 2.8498655e-11,
                "harassment/threatening": 0.63055265,
                "violence": 0.99011886,
              }
            }
          ]
        });

        let response: CreateResponse = serde_json::from_value(json).unwrap();

        let expectation = CreateResponse {
            id: "modr-XXXXX".to_string(),
            model: "text-moderation-005".to_string(),
            results: vec![ModerationObject {
                flagged: true,
                categories: Categories {
                    sexual: false,
                    hate: false,
                    harassment: false,
                    self_harm: false,
                    sexual_minors: false,
                    hate_threatening: false,
                    violence_graphic: false,
                    self_harm_intent: false,
                    self_harm_instructions: false,
                    harassment_threatening: true,
                    violence: true,
                },
                category_scores: CategoryScores {
                    sexual: 1.228_207_1e-6,
                    hate: 0.010696256,
                    harassment: 0.29842457,
                    self_harm: 1.523_692_5e-8,
                    sexual_minors: 5.724_626_8e-8,
                    hate_threatening: 0.0060676364,
                    violence_graphic: 4.435_014e-6,
                    self_harm_intent: 8.098441e-10,
                    self_harm_instructions: 2.8498655e-11,
                    harassment_threatening: 0.63055265,
                    violence: 0.99011886,
                },
            }],
        };

        assert_eq!(response, expectation);
    }
}
