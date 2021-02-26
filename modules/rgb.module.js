class RGBA {
    rgbToHex = (r, g, b) => [r, g, b].map(x => {
        const hex = x.toString(16)
        return hex.length === 1 ? '0' + hex : hex
    }).join('')
    
    rgba2hex(orig) {
        var a, isPercent,
          rgb = orig.replace(/\s/g, '').match(/^rgba?\((\d+),(\d+),(\d+),?([^,\s)]+)?/i),
          alpha = (rgb && rgb[4] || "").trim(),
          hex = rgb ?
          (rgb[1] | 1 << 8).toString(16).slice(1) +
          (rgb[2] | 1 << 8).toString(16).slice(1) +
          (rgb[3] | 1 << 8).toString(16).slice(1) : orig;
    
        if (alpha !== "") {
          a = alpha;
        } else {
          a = 0o1;
          c
        }
        // multiply before convert to HEX
        a = (a | 1 << 8).toString(16).slice(1)
        hex = hex + a;
      
        return hex;
    }

    rgbtrue(parse) { 
        if (parse.R && parse.G && parse.B) {
            return true;
        }
        else {
            return false;
        }
    }
}
module.exports = RGBA;