class ObjHandler {
    getDifference(newObject, oldObject) {
        let arr1 = Object.keys(newObject.Ableton[Object.keys(newObject.Ableton)[1]][0])
        let arr2 = Object.keys(oldObject.Ableton[Object.keys(oldObject.Ableton)[1]][0])
        let difference = arr1.filter(x => !arr2.includes(x));
        return difference;
    }
}
module.exports = ObjHandler;