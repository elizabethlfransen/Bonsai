import { fetchXML } from "@/utils/fetch.ts";

interface MavenVersions {
  metadata: {
    versioning: {
      latest: string;
      release?: string;
      versions: {
        version: string[];
      };
    };
  };
}

export interface ModLoader {
  id: string;
  name: string;
  getVersionList(mcVersion: string): Promise<ModLoaderVersions>;
}

export interface ModLoaderVersions {
  latest?: string;
  release?: string;
  versions: string[];
}

export const MOD_LOADERS = [
  {
    id: "neoforge",
    name: "NeoForge",
    async getVersionList(version) {
      if (version === "1.20.1") {
        return await fetchForgeStyle(
          version,
          "https://maven.neoforged.net/releases/net/neoforged/forge/maven-metadata.xml",
          "NeoForge",
        );
      } else if (version.startsWith("1.")) {
        return await fetchOldNeoForgeStyle(version);
      } else {
        return fetchNeoForgeStyle(version);
      }
    },
  },

  {
    id: "fabric",
    name: "Fabric",
    getVersionList: () =>
      fetchMavenVersionsWithFilterMap(
        "https://maven.fabricmc.net/net/fabricmc/fabric-loader/maven-metadata.xml",
        "Fabric",
      ),
  },
  {
    id: "forge",
    name: "Forge",
    getVersionList: (version) => fetchForgeStyle(version),
  },
  {
    id: "liteloader",
    name: "LiteLoader",
    getVersionList: (version) =>
      fetchMavenVersionsWithFilterMap(
        "https://repo.mumfrey.com/content/repositories/snapshots/com/mumfrey/liteloader/maven-metadata.xml",
        "LiteLoader",
        (v) => v === `${version}-SNAPSHOT` ? v : undefined,
      ),
  },
  {
    id: "quilt",
    name: "Quilt",
    getVersionList: () =>
      fetchMavenVersionsWithFilterMap(
        "https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/maven-metadata.xml",
        "Quilt",
      ),
  },
] satisfies ModLoader[];

async function fetchMavenVersionsWithFilterMap(
  url: string,
  name: string,
  parseVersion: (mavenVersion: string) => string | undefined = (v) => v,
): Promise<ModLoaderVersions> {
  const xml = await fetchXML<MavenVersions>(url, `${name} versions`);
  return {
    latest: parseVersion(xml.metadata.versioning.latest!),
    release: xml.metadata.versioning.release &&
      parseVersion(xml.metadata.versioning.release),
    versions: xml.metadata.versioning.versions.version.map(parseVersion).filter(
      (v) => v,
    ).reverse() as string[],
  };
}

const fetchForgeStyle = (
  filteredMcVersion: string,
  url: string =
    "https://files.minecraftforge.net/maven/net/minecraftforge/forge/maven-metadata.xml",
  name: string = "Forge",
) =>
  fetchMavenVersionsWithFilterMap(url, name, (version: string) => {
    const [mcVersion, loaderVersion] = version.split("-");
    if (!loaderVersion || mcVersion != filteredMcVersion) return undefined;
    return loaderVersion;
  });

const fetchOldNeoForgeStyle = (filteredMcVersion: string) => {
  const [mcMajor, mcMinor, mcBuild] = filteredMcVersion.split(".");
  if (!mcMajor || !mcMinor || !mcBuild) {
    throw new Error(
      `Invalid version format: "${filteredMcVersion}". Expected "major.minor.build".`,
    );
  }
  const versionPrefix = `${mcMinor}.${mcBuild}.`;
  return fetchMavenVersionsWithFilterMap(
    "https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml",
    "NeoForge",
    (version: string) => {
      if (!version.startsWith(versionPrefix)) {
        return undefined;
      }
      return version;
    },
  );
};

function splitPrerelease(str: string): [string, string] {
  const index = str.indexOf("-");
  if (index === -1) {
    return [str, ""];
  } else {
    return [
      str.slice(0, index),
      str.slice(index + 1),
    ];
  }
}

const fetchNeoForgeStyle = (filteredMcVersion: string) => {
  let [mcYear, mcMajor, mcMinor] = filteredMcVersion.split(".");
  let mcPreRelease = "";
  if (!mcYear || !mcMajor) {
    throw new Error(
      `Invalid version format: "${filteredMcVersion}". Expected "major.minor.build".`,
    );
  }

  if (mcMinor) {
    [mcMinor, mcPreRelease] = splitPrerelease(mcMinor);
  } else {
    mcMinor = "0";
    [mcMajor, mcPreRelease] = splitPrerelease(mcMajor);
  }

  const requiredPrefix = `${mcYear}.${mcMajor}.${mcMinor}.`;
  const requiredSuffix = mcPreRelease ? `+${mcPreRelease}` : "";
  return fetchMavenVersionsWithFilterMap(
    "https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml",
    "NeoForge",
    (version: string) => {
      if (
        version.startsWith(requiredPrefix) && version.endsWith(requiredSuffix)
      ) {
        return version;
      }
      return undefined;
    },
  );
};
