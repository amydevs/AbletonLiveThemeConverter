# Ableton Live Theme Converter (AltC)
## About
AltC is a command line tool (currently) built in NodeJS that converts Ableton Live <=10 themes to the new format of Ableton Live 11 themes. It does this by updating the schema of the XML files, as well as translating all the original RGBA values to hex values.

## Basic Usage
|Info|Command|
|-|-|
|Usage:                     |`AbletonThemeConverter.exe {Input Theme from Old Version} {Input Theme from New Version} {Output File Name}`|
|Example:                   |`AbletonThemeConverter.exe 00Light.ask 00Lightnew.ask newtheme.ask`|
|Example with Directories:  |`AbletonThemeConverter.exe ./themes/00Light.ask ./themes/00Lightnew.ask ./themes/newtheme.ask`|
### Note:                       Please make sure that the filenames have '.ask' on the end<br>

## Usage if Second Input File is Unavailable
More Notes:                 If you do not enter a valid path for the {Input Theme from New Version} argument, missing parts of your converted theme will be automatically filled in by parts of the Ableton Live 11 00Light theme. <br>

|Info|Command|
|-|-|
|Example:                   |`AbletonThemeConverter.exe 00Light.ask TypeInAnyInvalidFileNameHere newtheme.ask`|
|Example with Directories:  |`AbletonThemeConverter.exe ./themes/00Light.ask hsdajshdajshdjashd ./themes/newtheme.ask`|

### I've only tested this for converting from Live 10 themes to Live 11 themes. Results may vary with different versions.
