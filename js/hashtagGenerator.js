function generateHashtag(string) {
    if (string.length === 0 || string === undefined) return false;
    let arrString = string.trim().toLowerCase().split(" ");

    let hashed = arrString.filter(val => val.trim().length > 0 && val !== undefined && val != " ");
    let str = hashed.map(sub => {
        if (sub.length >= 1)
            return sub[0].toUpperCase() + sub.substring(1);
        return sub;
    })
    if (str.join('').trim().length < 1) return false;
    str = "#" + str.join('').trim();
    if (str.trim().length > 140) return false;

    return str;
}

module.exports = generateHashtag;