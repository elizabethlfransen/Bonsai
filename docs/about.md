## Vision

This project has 5 main goals.

- Managing a modpack that can be deployed to both Modrinth and Curseforge.
- Updating mods to latest versions in a modpack.
- Adding mods should be as easy as typing a command or clicking a button
- This project should expect users can commit their project into version control
- Users should be able to see mods they care care about at a glance and see a full modlist if they desire.

## User Stories & Targets

### MVP (0.1.0 Release. Goal: Build and Release a modrinth modpack)
> As a **Modpack Creator**,<br>
> I want to **initialize a new project**,<br>
> so that **I can quickly set up the required file structure and start building my pack**.

> As a **Modpack Creator**,<br>
> I want to **remove a mod from my project**,<br>
> so that **it is completely cleaned up and excluded from future builds**.

> As a **Modpack Creator**,<br>
> I want to **validate my project files**,<br>
> so that **I can catch compatibility errors or missing dependencies before trying to share it**.

> As a **Modpack Creator**,<br>
> I want to **add a modrinth mod to my project**,<br>
> so that **it is automatically tracked and included in the modpack manifest**.

> As a **Modpack Creator**,<br>
> I want to **export my project into Modrinth compatible format**,<br>
> so that **I can manually upload it to Modrinth**.

> As a **Modpack Creator**,<br>
> I want to **be able to list mods that I've explicitly added to my pack**,<br>
> so that **I can audit the mods I've added**.

> As a **Modpack Creator**,<br>
> I want to **be able to list all mods that are included in my pack**,<br>
> so that **I can audit the full mod list including depedencies**.

### 0.2 Release (Goal: Build and release a CurseForge modpack)

> As a **Modpack Creator**,<br>
> I want to **export my project into Curseforge compatible format**,<br>
> so that **I can manually upload it to CurseForge**.

> As a **Modpack Creator**,<br>
> I want to **link CurseForge mods and modrinth mods together**,<br>
> so that **I can include the respective mods in each format**.

> As a **Modpack Creator**,<br>
> I want to **automatically link CurseForge mods and modrinth mods together when adding them**,<br>
> so that **I don't have to link them manually after the fact**.

> As a **Modpack Creator**,<br>
> I want to **add Modrinth and CurseForge exclusive mods**,<br>
> so that **I can have mods that are only included in respective modpacks**.

### 0.3 Release (Goal: Updating and testing)

> As a **Modpack Creator**,<br>
> I want to **update my mod loader**,<br>
> so that **I can have the latest up to date version for my modpack**.

> As a **Modpack Creator**,<br>
> I want to **update my mods**,<br>
> so that **I can have the latest up to date versions for my modpack**.

> As a **Modpack Creator**,<br>
> I want to **serve my project locally**,<br>
> so that **I can easily test the modpack in Prism Launcher before publishing it**.

### 1.0 Release (Goal: Web & Nice To Haves)

> As a **Modpack Creator**,<br>
> I want to **host a local web interface**,<br>
> so that **I have the option to manage my project via a visual UI instead of just the CLI**.

> As a **Modpack Creator**,<br>
> I want to **directly publish/distribute my modpack through the tool**,<br>
> so that **I can skip manual uploads and automate my release workflow**.