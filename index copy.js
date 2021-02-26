const fs = require('fs');
const xml2js = require('xml2js');

const rgba = new (require('./modules/rgb.module'));
const FileHandler = new (require('./modules/filehandler.module'));
const ObjHandler = new (require('./modules/objhandler.module'));

const oldxmltext = FileHandler.readFile('Vice10.ask', 'utf8');
var oldObject = FileHandler.xml2obj(oldxmltext)
const newxmltext = FileHandler.readFile('00Lightnew.ask', 'utf8');
var newObject = FileHandler.xml2obj(newxmltext)

console.log(ObjHandler.getDifference(newObject, oldObject))

// xml2js.parseString(`<start>
// </start>`
// ,(err, data)=> {
//     Object.keys(data.start).forEach(e=>{
//         console.log(`{'${e}': ${JSON.stringify(data.start[e])}},`)
//     })    
// })


xml2js.parseString(oldxmltext, (err, xmlobj) => {
    //rename to theme
    xmlobj.Ableton.Theme = xmlobj.Ableton.SkinManager;
    delete xmlobj.Ableton.SkinManager;

    //edit rgb values
    ObjectKeys = Object.keys(xmlobj.Ableton.Theme[0]);
    ObjectKeys.forEach(element => {
        var object = xmlobj.Ableton.Theme[0][element][0]
        if(rgba.rgbtrue(object)) {
            var hex = rgba.rgba2hex(`rgba(${object.R[0].$.Value}, ${object.G[0].$.Value}, ${object.B[0].$.Value}, ${object.Alpha[0].$.Value})`)
            xmlobj.Ableton.Theme[0][element][0]={}
            xmlobj.Ableton.Theme[0][element][0].$ = {'Value': '#'+hex}
        }
    });

    //edit meta values
    xmlobj.Ableton.$.MinorVersion = "11.0_432";
    xmlobj.Ableton.$.SchemaChangeCount = "3";
    xmlobj.Ableton.$.Creator="Ableton Live 11.0.1d1";
    
    //add new values
    var newValues = [
        {'SurfaceAreaForeground': [{ '$':{ 'Value':'#000000' } }] },
        {'SceneContrast': [{ '$':{ 'Value':'#000000' } }] },
        {'SelectionBackgroundContrast': [{ '$':{'Value':'#a8d2e0' } }] },
        {'TakeLaneTrackHighlighted': [{ '$':{'Value':'#d4d4d4' } }] },
        {'TakeLaneTrackNotHighlighted': [{ '$':{ 'Value':'#b4b4b4' } }] },
        {'DimmedWaveformColor': [{ '$':{ 'Value':'#777777' } }] },
        {'VelocityColor': [{ '$':{ 'Value':'#f04e42' } }] },
        {'VelocitySelectedOrHovered': [{ '$':{'Value':'#6da7ff'} }] },
        {'NoteProbability': [{ '$':{ 'Value':'#000000' } }] },
        {'ClipBorderAlpha': [{"$":{"Value":"64"}}]},
        {'ScrollBarAlpha': [{"$":{"Value":"128"}}]},
        {'ScrollBarOnHoverAlpha': [{"$":{"Value":"255"}}]},
        {'ScrollBarBackgroundAlpha': [{"$":{"Value":"63"}}]},
        {'InaudibleTakeLightness': [{"$":{"Value":"0.560000000000000053"}}]},
        {'InaudibleTakeSaturation': [{"$":{"Value":"0.330000000000000016"}}]},
        {'InaudibleTakeNameLightness': [{"$":{"Value":"0.92000000000000004"}}]},
        {'InaudibleTakeNameSaturation': [{"$":{"Value":"1"}}]},
        {'AutomationLaneClipBodyLightness': [{"$":{"Value":"0.510000000000000009"}}]},
        {'AutomationLaneClipBodySaturation': [{"$":{"Value":"0.75"}}]},
        {'AutomationLaneHeaderLightness': [{"$":{"Value":"0.589999999999999969"}}]},
        {'AutomationLaneHeaderSaturation': [{"$":{"Value":"0.770000000000000018"}}]},
        {'TakeLaneHeaderLightness': [{"$":{"Value":"0.569999999999999951"}}]},
        {'TakeLaneHeaderSaturation': [{"$":{"Value":"0.770000000000000018"}}]},
        {'TakeLaneHeaderNameLightness': [{"$":{"Value":"0.949999999999999956"}}]},
        {'TakeLaneHeaderNameSaturation': [{"$":{"Value":"1"}}]},
        {'AutomationLaneHeaderNameLightness': [{"$":{"Value":"0.200000000000000011"}}]},
        {'AutomationLaneHeaderNameSaturation': [{"$":{"Value":"1"}}]},
        {'ClipContrastColorAdjustment': [{"$":{"Value":"20"}}]},
        {'MutedAuditionClip': [{ '$': { Value: '#bababa' } }] },
        {'LinkedTrackHover': [{ '$': { Value: '#b0e3ff' } }] },
        {'LinkedTrackHover': [{"$":{"Value":"#b0e3ff"}}]},
        {'ExpressionLaneHeaderHighlight': [{"$":{"Value":"#d6d6d6"}}]},
        {'ZoomPanHandle': [{"$":{"Value":"#262626"}}]},
        {'GridLineBase': [{"$":{"Value":"#00000031"}}]},
        {'StandardVuMeter': [{"OnlyMinimumToMaximum":[{"$":{"Value":"false"}}],"Maximum":[{"$":{"Value":"#ff0a0a"}}],"AboveZeroDecibel":[{"$":{"Value":"#ffd00a"}}],"ZeroDecibel":[{"$":{"Value":"#c6f864"}}],"Minimum":[{"$":{"Value":"#0af864"}}]}]},
        {'OverloadVuMeter': [{"OnlyMinimumToMaximum":[{"$":{"Value":"true"}}],"Maximum":[{"$":{"Value":"#ff0a0a"}}],"AboveZeroDecibel":[{"$":{"Value":"#ffffff"}}],"ZeroDecibel":[{"$":{"Value":"#ffffff"}}],"Minimum":[{"$":{"Value":"#af0a0a"}}]}]},
        {'DisabledVuMeter': [{"OnlyMinimumToMaximum":[{"$":{"Value":"false"}}],"Maximum":[{"$":{"Value":"#ff0a0a"}}],"AboveZeroDecibel":[{"$":{"Value":"#ffd00a"}}],"ZeroDecibel":[{"$":{"Value":"#828282"}}],"Minimum":[{"$":{"Value":"#6e6e6e"}}]}]},
        {'HeadphonesVuMeter': [{"OnlyMinimumToMaximum":[{"$":{"Value":"false"}}],"Maximum":[{"$":{"Value":"#a5a5f1"}}],"AboveZeroDecibel":[{"$":{"Value":"#90aaec"}}],"ZeroDecibel":[{"$":{"Value":"#90aaec"}}],"Minimum":[{"$":{"Value":"#0affff"}}]}]},
        {'SendsOnlyVuMeter': [{"OnlyMinimumToMaximum":[{"$":{"Value":"false"}}],"Maximum":[{"$":{"Value":"#c8c800"}}],"AboveZeroDecibel":[{"$":{"Value":"#c8c800"}}],"ZeroDecibel":[{"$":{"Value":"#6464ff"}}],"Minimum":[{"$":{"Value":"#6464ff"}}]}]},
        {'BipolarGainReductionVuMeter': [{"OnlyMinimumToMaximum":[{"$":{"Value":"false"}}],"Maximum":[{"$":{"Value":"#5577c6"}}],"AboveZeroDecibel":[{"$":{"Value":"#5577c6"}}],"ZeroDecibel":[{"$":{"Value":"#ffa519"}}],"Minimum":[{"$":{"Value":"#ffa519"}}]}]},
        {'OrangeVuMeter': [{"OnlyMinimumToMaximum":[{"$":{"Value":"true"}}],"Maximum":[{"$":{"Value":"#ffa519"}}],"AboveZeroDecibel":[{"$":{"Value":"#ffa519"}}],"ZeroDecibel":[{"$":{"Value":"#ffa519"}}],"Minimum":[{"$":{"Value":"#ffa519"}}]}]},
    ]
    newValues.forEach(element => {
        var elementKey = Object.keys(element)[0]
        xmlobj.Ableton.Theme[0][elementKey] = element[elementKey];
    })

    //convert back to xml
    var builder = new xml2js.Builder();
    var xml = builder.buildObject(xmlobj);
    fs.writeFile('helloworld.ask', xml, function (err) {
        if (err) return console.log(err);
    });
});




  