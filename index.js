//import modules
const rgba = new (require('./modules/rgb.module'));
const FileHandler = new (require('./modules/filehandler.module'));
const ObjHandler = new (require('./modules/objhandler.module'));

//define args
var args = process.argv.slice(2);
var oldinputname = args[0]
var newinputname = args[1]
var newoutputname = args[2]
console.log(args)

try {
    //initOldObjects
    const oldxmltext = FileHandler.readFile(oldinputname, 'utf8');
    var oldObject = FileHandler.xml2obj(oldxmltext)
    var oldObjectInnerReference = oldObject.Ableton[Object.keys(oldObject.Ableton)[1]][0]

    //initNewObjects
    const newxmltext = FileHandler.readFile(newinputname, 'utf8');
    var newObject = FileHandler.xml2obj(newxmltext)
    var newObjectInnerReference = newObject.Ableton[Object.keys(newObject.Ableton)[1]][0]
}
catch(err) {
    console.error(err)
    process.exit(1)
}


//rename to new name
oldObject.Ableton[Object.keys(newObject.Ableton)[1]] = oldObject.Ableton[Object.keys(oldObject.Ableton)[1]];
delete oldObject.Ableton[Object.keys(oldObject.Ableton)[1]];

//edit rgb values
ObjectKeys = Object.keys(oldObject.Ableton.Theme[0]);
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
    newobj[e] = newObjectInnerReference[e]
    // var string = `{'${e}': ${JSON.stringify(newObjectInnerReference[e])}},`
    newAddedValues.push(newobj);
})

newAddedValues.forEach(element => {
    var elementKey = Object.keys(element)[0]
    oldObject.Ableton.Theme[0][elementKey] = element[elementKey];
})

//convert back to xml
FileHandler.saveFile(newoutputname, FileHandler.obj2xml(oldObject))





  