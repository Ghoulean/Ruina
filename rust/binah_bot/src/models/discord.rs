use serde::Deserialize;
use serde::Serialize;
use serde_repr::*;

/**
 * Object holding embed data.
 * 
 * See also: https://discord.com/developers/docs/resources/channel#embed-object
 */
#[derive(Serialize, Deserialize)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<i32>,
    pub image: Option<DiscordEmbedImage>,
    pub footer: Option<DiscordEmbedFooter>,
    pub author: Option<DiscordEmbedAuthor>,
    pub fields: Vec<DiscordEmbedFields>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordEmbedImage {
    pub url: String
}

#[derive(Serialize, Deserialize)]
pub struct DiscordEmbedFooter {
    pub text: String,
    pub icon_url: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct DiscordEmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct DiscordEmbedFields {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>
}


/**
 * Object holding interaction data. This only reflects the information that the bot cares about; other fields are discarded.
 *
 * See also: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
 */
#[derive(Serialize, Deserialize)]
pub struct DiscordInteraction {
    pub id: String,
    pub r#type: DiscordInteractionType,
    pub token: String,
    pub application_id: String,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    pub channel_id: Option<String>,
    pub data: Option<DiscordInteractionData>,
    pub metadata: DiscordInteractionMetadata
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum DiscordInteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5
}

#[derive(Serialize, Deserialize)]
pub struct DiscordInteractionData {
    pub id: String,
    pub name: String,
    pub options: Vec<DiscordInteractionOptions>,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordInteractionOptions {
    pub name: String,
    pub value: String
}

#[derive(Serialize, Deserialize)]
pub struct DiscordInteractionMetadata {
    pub timestamp: String,
    pub signature: String,
    pub json_body: String,
}

// Workaround: https://github.com/serde-rs/serde/issues/745
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiscordInteractionResponse {
    MessageResponse {
        r#type: DiscordInteractionResponseType,
        data: Option<DiscordInteractionResponseMessage>
    },
    AutocompleteResponse {
        r#type: DiscordInteractionResponseType,
        data: Option<DiscordInteractionResponseAutocomplete>
    },
    PingResponse {
        r#type: DiscordInteractionResponseType,
    }
}


#[derive(Serialize, Deserialize)]
pub struct DiscordInteractionResponseMessage {
    embeds: Option<Vec<DiscordEmbed>>
}

#[derive(Serialize, Deserialize)]
pub struct DiscordInteractionResponseAutocomplete {
    choices: Option<Vec<DiscordInteractionOptions>>
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum DiscordInteractionResponseType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
    PremiumRequired = 10,
}

