const rgba = new (require('./modules/rgb.module'));
const FileHandler = new (require('./modules/filehandler.module'));
const ObjHandler = new (require('./modules/objhandler.module'));

//initOldObjects
const oldxmltext = FileHandler.readFile('Vice10.ask', 'utf8');
var oldObject = FileHandler.xml2obj(oldxmltext)
var oldObjectInnerReference = oldObject.Ableton[Object.keys(oldObject.Ableton)[1]][0]

//initNewObjects
const newxmltext = FileHandler.readFile('00Lightnew.ask', 'utf8');
var newObject = FileHandler.xml2obj(newxmltext)
var newObjectInnerReference = newObject.Ableton[Object.keys(newObject.Ableton)[1]][0]

//rename to theme
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
FileHandler.saveFile('helloworld.ask', FileHandler.obj2xml(oldObject))





  