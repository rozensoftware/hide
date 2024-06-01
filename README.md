# Hide

Version: 0.1.1

This is a command that can help you to copy and autorun your payload or program in Windows OS.

## Purpose

The hide command can serve as a tool to copy, hide, and autorun your malware, virus, or other program in Windows OS.
Administrator privileges are required to successfully run the program.

## Features

- Copies a payload to the System32 folder
- Autoruns a payload by writing its path to the registry
- Copies a payload to the Menu Autostart folder
- Creates a task in Windows Task Manager to autorun a payload
- Runs a payload after installing it

## Building

Rust compiler is needed to build the command.

## Examples

Copy a payload to the System32 folder and write its path to the run node of the Windows Registry

```bash
./hide -f virus.exe -o 1
```

Copy a payload to the Autostart System Menu folder

```bash
./hide -f virus.exe -o 2
```

..and run it:

```bash
./hide -f virus.exe -o 2 -r
```

Copy a payload to the System32 folder and create an autorun task in The Windows Task Manager.
For this to work you have to have the *autorun.ps1* script in the same folder as the hide.exe file.

```bash
./hide -f virus.exe -o 3
```

## License

This project is licensed under

MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>).

## Contributing / Feedback

I am quite new to Rust. I am always glad to learn from anyone.
If you want to contribute, you are more than welcome to be a part of the project! Try to share you thoughts first! Feel free to open a new issue if you want to discuss new ideas.

Any kind of feedback is welcome!
