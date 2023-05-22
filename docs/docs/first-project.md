---
sidebar_position: 3
---

# Creating your First Project

For a first project we will create a Sega Genesis / Mega Drive project that is using the SGDK (Docker) as the SDK, and we will then build the project and run it in a supported emulator.

## Creating the Project

To create a project you need to run the command `retrodevhelper init`, this will display some prompts for the user to fill in. The first one is the project name which will be `first-project`, next it will be console which we will select `Genesis/Mega Drive`, and finally SDK which will be `SGDK`. After selecting all the values your should have an output that looks like the one below.

```bash
➜  ~ retrodevhelper init

█▀█ █▀▀ ▀█▀ █▀█ █▀█ ▄▄ █▀▄ █▀▀ █░█ ▄▄ █░█ █▀▀ █░░ █▀█ █▀▀ █▀█
█▀▄ ██▄ ░█░ █▀▄ █▄█ ░░ █▄▀ ██▄ ▀▄▀ ░░ █▀█ ██▄ █▄▄ █▀▀ ██▄ █▀▄

> Project Name: first-project
> Console Target: Genesis/Mega Drive
> Select a SDK: SGDK
Creating Project...
Project Created
```

If we look in the file explorer under this new directory we will find the following structure

```
- first-project/ (Project Folder)
  - .vscode/ (Visual Studio Code Project Files)
  - out/ (Where the rom will be placed on build)
  - res/ (Where the resources (textures, etc...) will be placed)
  - src/ (Where the source code is located)
  - project.json (The retrodevhelper project file)
```

## Building the Project

Running the following command will build the project.

```bash
retrodevhelper build
```

For SGDK it will spin up a docker image that will build the project and output the rom.bin file in the test-project/out directory. This file can be used in any emulator or flash cart but we will use the run command next to run the project easily.

## Running the Project

The project can be run with the following command, it should load the emulator of choice with the built project.

```bash
retrodevhelper run
```
