//import modules
const rgba = new (require('./modules/rgb.module'));
const FileHandler = new (require('./modules/filehandler.module'));
const ObjHandler = new (require('./modules/objhandler.module'));
const { version } = require('../package.json');

//define args
var args = process.argv.slice(2);
var oldinputname = args[0]
var newinputname = args[1]
var newoutputname = args[2]

//Help argument handling
if(args[0] == "-help" || args[0] == "--help" || args[0] == "-h" || args[0] == "--h" || !args[0]) {
console.log(
`Ableton Live Theme Converter (AltC) - Version ${version}
Usage:                      AbletonThemeConverter.exe {Input Theme from Old Version} {Input Theme from New Version} {Output File Name} 

Example:                    AbletonThemeConverter.exe 00Light.ask 00Lightnew.ask newtheme.ask
Example with Directories:   AbletonThemeConverter.exe ./themes/00Light.ask ./themes/00Lightnew.ask ./themes/newtheme.ask

Note:                       Please make sure that the filenames have '.ask' on the end.`
)
process.exit(0)
}

//Init objects from xml
try {
    //initOldObjects
    const oldxmltext = FileHandler.readFile(oldinputname, 'utf8');
    var oldObject = FileHandler.xml2obj(oldxmltext)
    var oldObjectInnerReference = oldObject.Ableton[Object.keys(oldObject.Ableton)[1]]
}
catch(err) {
    console.error(err)
    process.exit(1)
}
try {
    //initNewObjects
    const newxmltext = FileHandler.readFile(newinputname, 'utf8');
    var newObject = FileHandler.xml2obj(newxmltext)
}
catch(err) {
    //If cannot be found, Live 11 Light theme will be used as the backup.
    var newObject = (new (require('./modules/polyfill/polyfillbackup.module'))).backupObject
    console.log('Notice: Your second input was invalid, your output theme will be polyfilled using the default Light Theme.')
}
var newObjectInnerReference = newObject.Ableton[Object.keys(newObject.Ableton)[1]]

//rename to new name
oldObject.Ableton[Object.keys(newObject.Ableton)[1]] = oldObjectInnerReference;
delete oldObject.Ableton[Object.keys(oldObject.Ableton)[1]];

//edit rgb values
var ObjectKeys = Object.keys(oldObject.Ableton.Theme[0]);
ObjectKeys.forEach(element => {
    var object = oldObject.Ableton.Theme[0][element][0]
    if(rgba.rgbtrue(object)) {
        var hex = rgba.rgba2hex(`rgba(${object.R[0].$.Value}, ${object.G[0].$.Value}, ${object.B[0].$.Value}, ${object.Alpha[0].$.Value})`)
        oldObject.Ableton.Theme[0][element][0]={}
        oldObject.Ableton.Theme[0][element][0].$ = {'Value': '#'+hex}
    }
});

//edit meta values
oldObject.Ableton.$.MinorVersion = newObject.Ableton.$.MinorVersion
oldObject.Ableton.$.SchemaChangeCount = newObject.Ableton.$.SchemaChangeCount
oldObject.Ableton.$.Creator=newObject.Ableton.$.Creator

//add new values
var newAddedValues = [];
ObjHandler.getDifference(newObject, oldObject).forEach(e=> {
    let newobj = {}
    newobj[e] = newObjectInnerReference[0][e]
    // var string = `{'${e}': ${JSON.stringify(newObjectInnerReference[e])}},`
    newAddedValues.push(newobj);
})
  
newAddedValues.forEach(element => {
    var elementKey = Object.keys(element)[0]
    oldObject.Ableton.Theme[0][elementKey] = element[elementKey];
})

//convert back to xml
FileHandler.saveFile(newoutputname, FileHandler.obj2xml(oldObject))
console.log(`Done! Output file in "${FileHandler.path.resolve(newoutputname)}"`)

//Polyfill Converter:
// const PolyFillHandler = new (require('./modules/polyfill/polyfillhandler.module'))
// FileHandler.saveFile('./src/modules/polyfill/backupJsons/00LightPolyfill.json', JSON.stringify(PolyFillHandler.polyfillConverter(newAddedValues), null, 2))






  