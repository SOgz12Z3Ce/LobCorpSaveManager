# LobCorpSaveManager

A tool for editing save files of the vanilla *Lobotomy Corporation* game.

## Features

- Edit save files without launching the game
- Command-line interface

## Requirements

- Windows (currently)
  
Due to the current build system, only Windows is supported for now. Support for other platforms is planned.

## Installation

### Download prebuilt binaries

#### Windows

1. Download the binary release from the Releases page.
2. Extract the archive and add the `lobcsm/bin` directory to your `PATH`.

### Build from source

Clone the repository:

```bash
git clone https://github.com/SOgz12Z3Ce/LobCorpSaveManager.git
cd LobCorpSaveManager
```

#### Windows

1. Install a Mono version capable of building and running .NET Framework 3.5 programs. One available option is **Mono 2.8.1** from the official archive: [https://download.mono-project.com/archive/2.8.1/windows-installer/3/mono-2.8.1-gtksharp-2.12.10-win32-3.exe](https://download.mono-project.com/archive/2.8.1/windows-installer/3/mono-2.8.1-gtksharp-2.12.10-win32-3.exe)
2. Add Mono’s `bin` directory to your `PATH`. For example (Mono 2.8.1):

```
C:\Program Files (x86)\Mono-2.8.1\bin
```

3. Build `LobCorpSaveSerializer`:

```bash
cd LobCorpSaveSerializer
xbuild LobCorpSaveSerializer.csproj
```

## Usage

To deserialize a save file into JSON:

```bash
lobcss <filename>
```

## Notes

* Some save files cannot be deserialized yet; support is planned.
* More features are under development.

## License

This project is licensed under the MIT License.

It redistributes third-party components:

- **Mono runtime**, licensed under the MIT License and the GNU LGPL
- **Newtonsoft.Json**, licensed under the MIT License

These components are not part of this project.

The source code of Mono is available at:
[https://github.com/mono/mono](https://github.com/mono/mono)
