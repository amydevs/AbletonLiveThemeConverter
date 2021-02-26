const fs = require('fs');
const xml2js = require('xml2js');

class FileHandler {
    readFile(filename) {
        return fs.readFileSync(filename, 'utf8');
    }
    xml2obj(xml) {
        var obj = {}
        xml2js.parseString(xml,(err, data)=> {
            obj = data
        })
        return obj;
    }
    obj2xml(obj) {
        var builder = new xml2js.Builder();
        var xml = builder.buildObject(obj);
        return xml;
    }
    saveFile(filename, xml) {
        fs.writeFile(filename, xml, function (err) {
            if (err) return console.log(err);
        });
    }
}
module.exports = FileHandler;