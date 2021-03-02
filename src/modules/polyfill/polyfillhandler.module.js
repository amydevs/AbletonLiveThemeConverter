class PolyFillHandler {
    constructor() {
      this.backupObject = {
      }
    }
    polyfillConverter(newAddedValues) {
        //Polyfill Converter
        let defaultObject = {
            "Ableton": {
                "$": {
                    "MajorVersion": "5",
                    "MinorVersion": "11.0_432",
                    "SchemaChangeCount": "3",
                    "Creator": "Ableton Live 11.0.1d1",
                    "Revision": ""
                }
            }
        }
        let newObjectofValues = newAddedValues.reduce((result, current) => {
            return Object.assign(result, current);
        }, {});
        defaultObject.Ableton['Theme'] = [newObjectofValues]
        return defaultObject;
    }
  }
  module.exports = PolyFillHandler;