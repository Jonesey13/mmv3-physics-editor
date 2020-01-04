# mmv3-physics-editor

A tool for changing the car types / physics of the playstation game MicroMachines V3
* If you want to change/edit the actual tracks you should checkout the [MMEd](https://github.com/RichardBradley/MMEd) tool

* This gui tool leverages the [webview](https://github.com/zserge/webview) library to build a minimal native GUI
* The basic code structure for the gui follows the [Compactor](https://github.com/Freaky/Compactor) project closely

## Usage
_This tool has only been tested on the European (PAL) version of the game_

Copy the target binary into the same folder as the editor with the name mmv3.bin
* The editor will modify the file in place so take a backup beforehand if you would like to avoid data loss

## Version History
* 0.1: Initial version. Reading of car types by track only

## Thanks
* Rich Bradley for documenting the MMV3 resources [here](http://www.bradders.org/MMs/php-mms.php)
* Nick Tomlinson for mentioning the cheat engine tool which helped me locate the values in the binary
* [Dege](http://dege.freeweb.hu/) for providing the dgvoodoo2 Direct3D shim & patched MMV3 PC executable (which prompted me to re-investigate the assembly code in the first place)
* Softwire MMs crew for testing the changes