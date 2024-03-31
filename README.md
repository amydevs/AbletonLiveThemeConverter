# Ableton Live Theme Converter (AltC)

## About

AltC is a command line tool built in Rust that converts any Live >= 10 theme to be compatible with any other Live version >= 10. It does this by updating the schema of the XML files, as well as translating between RGBA values to hex color values.

## Basic Usage

|Info|Command|
|-|-|
|Usage:                     |`altc.exe [Input Theme from Old Version] --to-version (Live Version)`|
|Example:                   |`altc.exe live10theme.ask --to-version 11`|
|Example with Directories:  |`altc.exe ./themes/live10theme.ask --to-version 11`|

Note: Please make sure that the filenames have '.ask' on the end.

### MacOS Code Signing

As I do not have an Apple developer account, because it is 100 AUD, I cannot sign the MacOS binaries.

This means, that if you are using an M1 based MacOS device, you will not be able to run the `altc.aarch64-apple-darwin` application.

To get around this, you can either:

- use the `altc.x86_64-apple-darwin` download instead
- or [disable SIP](https://developer.apple.com/documentation/security/disabling_and_enabling_system_integrity_protection) (scary!)

## Reporting Issues

Please send me emails at [ayanamydev@gmail.com](mailto:ayanamydev@gmail.com) if you have any problems or suggestions for improvements
