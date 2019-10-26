const { execSync } = require('child_process');
const fs = require('fs');

module.exports = class File {

    constructor(path) {

        this.path = path;
        this.mime = null;

        if (!fs.existsSync(this.path)) {
            throw new Error(`${this.path} does not exist`);
        }

        let res = execSync(`file ${this.path} --mime-type`).toString('utf8');

        let mime = res.split(': ')[1].trim();
        if (mime.length) {
            this.mime = mime;
        } else {
            throw new Error('Error getting mime-type');
        }

    }

    read() {

        return fs.readFileSync(this.path)

    }

}
