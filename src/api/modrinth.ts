import { USER_AGENT } from "@/utils/fetch.ts";
import {GenericModrinthClient} from "@modrinth/api-client";
export const MODRINTH_CLIENT = new GenericModrinthClient({
    userAgent: USER_AGENT,
});

