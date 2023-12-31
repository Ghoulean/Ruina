import { SecretsManager } from "@aws-sdk/client-secrets-manager";
import {
    APIGatewayProxyEvent,
    APIGatewayProxyResult,
    Callback,
    Context,
} from "aws-lambda";
import { DataAccessor } from "./accessor/data_accessor";
import { SecretsAccessor } from "./accessor/secrets_accessor";
import { BinahBotCommand } from "./command/binahbot_command";
import { LorAutocomplete } from "./command/lor_autocomplete";
import { LorCommand } from "./command/lor_command";
import { IndexHandler } from "./index_handler";
import { AutocompleteRouter } from "./router/autocomplete_router";
import { CommandRouter } from "./router/command_router";
import { InteractionPayloadRouter } from "./router/interaction_payload_router";
import { ApiTransformer } from "./transformers/api_transformer";
import { EmbedTransformer } from "./transformers/embed_transformer";
import { InteractionResponseBuilder } from "./transformers/interaction_response_builder";
import { RequestTransformer } from "./transformers/request_transformer";
import {
    CLIENT_ID_ENV_KEY,
    S3_BUCKET_NAME_ENV_KEY,
    SECRETS_ID_ENV_KEY,
} from "./util/constants";
import { EnvVarRetriever } from "./util/env_var_retriever";

const envVarRetriever: EnvVarRetriever = new EnvVarRetriever();
const secretsManager: SecretsManager = new SecretsManager({});

const dataAccessor: DataAccessor = new DataAccessor();
const secretsAccessor: SecretsAccessor = new SecretsAccessor(
    envVarRetriever.getRequired(SECRETS_ID_ENV_KEY),
    secretsManager
);

const apiTransformer: ApiTransformer = new ApiTransformer();
const embedTransformer: EmbedTransformer = new EmbedTransformer(
    envVarRetriever.getRequired(S3_BUCKET_NAME_ENV_KEY)
);
const interactionResponseBuilder: InteractionResponseBuilder =
    new InteractionResponseBuilder();
const requestTransformer: RequestTransformer = new RequestTransformer();

const lorCommand: LorCommand = new LorCommand(dataAccessor, embedTransformer);
const lorAutocomplete: LorAutocomplete = new LorAutocomplete(dataAccessor);
const binahBotCommand: BinahBotCommand = new BinahBotCommand(
    envVarRetriever.getRequired(CLIENT_ID_ENV_KEY)
);

const commandRouter: CommandRouter = new CommandRouter(
    lorCommand,
    binahBotCommand
);
const autocompleteRouter: AutocompleteRouter = new AutocompleteRouter(
    lorAutocomplete
);

const interactionPayloadRouter: InteractionPayloadRouter =
    new InteractionPayloadRouter(
        interactionResponseBuilder,
        requestTransformer,
        commandRouter,
        autocompleteRouter,
        secretsAccessor
    );

const indexHandler: IndexHandler = new IndexHandler(
    apiTransformer,
    interactionPayloadRouter
);

export async function handler(
    event: APIGatewayProxyEvent,
    _context: Context,
    _callback: Callback
): Promise<APIGatewayProxyResult> {
    return indexHandler.handle(event);
}
