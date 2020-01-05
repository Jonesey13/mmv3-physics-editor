# mmv3-physics-editor

A tool for changing the car types / physics of the playstation game Micro Machines V3 (copyright belongs to Codemasters)
* If you want to change/edit the actual tracks you should checkout the [MMEd](https://github.com/RichardBradley/MMEd) tool

* This gui tool leverages the [webview](https://github.com/zserge/webview) library to build a minimal native GUI
* The basic code structure for the gui follows the [Compactor](https://github.com/Freaky/Compactor) project closely

## Build Instructions
To build the app please install rust (via rustup) and run `cargo build --release` from the root directory

## Usage
_This tool has only been tested on the European (PAL) version of the game_

Copy the target binary into the same folder as the editor with the name mmv3.bin
* The editor will modify the file in place so take a backup beforehand if you would like to avoid data loss

**This application will not work in administrator mode due to [this issue](https://github.com/windows-toolkit/Microsoft.Toolkit.Win32/issues/50)**!

## Debugging
There is a VS code task for debugging the rust code (which should just work with the LLDB debugger).

For the frontend you can run the [Edge DevTools](https://docs.microsoft.com/en-us/microsoft-edge/devtools-guide).

## Version History
* **0.1**: Initial version. Reading/Writing of car types only

## Thanks
* Rich Bradley for documenting the MMv3 resources [here](http://www.bradders.org/MMs/php-mms.php)
* Nick Tomlinson for mentioning the cheat engine tool which helped me locate the values in the binary
* [Dege](http://dege.freeweb.hu/) for providing the dgvoodoo2 Direct3D shim & patched MMV3 PC executable (which prompted me to re-investigate the assembly code in the first place)
* Softwire MMs crew for testing the changes