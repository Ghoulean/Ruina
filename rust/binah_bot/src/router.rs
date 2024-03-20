use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionResponse;
use crate::models::discord::DiscordInteractionResponse::PingResponse;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordInteractionType;

#[derive(strum::Display)]
#[strum(serialize_all = "lowercase")]
enum CommandName {
    Lor
}

pub fn get_response(discord_interaction: DiscordInteraction) -> DiscordInteractionResponse {
    // switch to static hashmap later, for now just use switch-case
    match (discord_interaction.r#type, discord_interaction.data) {
        (DiscordInteractionType::Ping, _) => {
            PingResponse {
                r#type: DiscordInteractionResponseType::Pong
            }
        }
        (DiscordInteractionType::ApplicationCommand, Some(data)) if data.name == CommandName::Lor.to_string() => {
            todo!()
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data)) if data.name == CommandName::Lor.to_string() => {
            todo!()
        }
        _ => panic!()
    }
}
