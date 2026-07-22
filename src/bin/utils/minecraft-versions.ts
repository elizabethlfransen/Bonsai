import { getMinecraftVersions } from "../../minecraft-version.ts";

export default async function () {
  console.log(await getMinecraftVersions());
}
