import { fetchJSON } from "./utils/fetch.ts";

export interface MCVersions {
    latest: {
        release: string,
        snapshot: string,
    },
    versions: MCVersion[]
}

export interface MCVersion {
    id: string,
    type: 'snapshot' | 'release',
    url: string,
    time: string,
    releaseTime: string
}

export const getMinecraftVersions = () => fetchJSON<MCVersions>("https://launchermeta.mojang.com/mc/game/version_manifest.json", "Minecraft versions");