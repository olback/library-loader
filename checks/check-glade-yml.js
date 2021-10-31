const xml = require('xml-js');
const yml = require('yaml');
const path = require('path');
const fs = require('fs');
const process = require('process');

const ymlPath = path.join(__dirname, '..', 'cli.yml');
const ymlData = fs.readFileSync(ymlPath, 'utf8').toString();
const ymlObject = yml.parse(ymlData);
const ymlFormats = ((data) => {
    for (let arg of data.args) {
        if (Object.keys(arg)[0] === 'format') {
            return arg.format.possible_values;
        }
    }
})(ymlObject);

const gladePath = path.join(__dirname, '..', 'll-gui', 'assets', 'library-loader.glade');
const gladeData = fs.readFileSync(gladePath, 'utf8').toString();
const gladeFormats = ((data) => {
    let reg = new RegExp('<item id="(?<id>.{3,})" translatable="(yes|no)">(.{3,20})<\\/item>$', 'gm');
    let arr = [];
    for (let line of [...data.matchAll(reg)]) {
        const { id } = line.groups;
        arr.push(id);
    }
    return arr;
})(gladeData);

const ymlSorted = ymlFormats.sort();
const gladeSorted = gladeFormats.sort();

if (ymlSorted.length !== gladeSorted.length) {
    console.log('Lengths does not match');
    console.log(ymlSorted, gladeSorted);
    process.exit(1);
}

for (let i = 0; i < ymlSorted.length; i++) {
    if (ymlSorted[i] !== gladeSorted[i]) {
        console.log(`${ymlSorted[i]} !== ${gladeSorted[i]}`);
        console.log(ymlSorted, gladeSorted);
        process.exit(1);
    }
}

console.log(ymlFormats, gladeFormats);
console.log('OK');
