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
}
module.exports = FileHandler;