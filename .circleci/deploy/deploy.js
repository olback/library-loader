#!/usr/bin/env node

const fs = require('fs');
const process = require('process');
const path = require('path');

const fetch = require('node-fetch');
const semver = require('semver');

const File = require('./file');

const gh = {
    url: process.env['GITHUB_API_URL'] || 'https://api.github.com/',
    user: process.env['CIRCLE_PROJECT_USERNAME'],
    repo: process.env['CIRCLE_PROJECT_REPONAME'],
    branch: process.env['CIRCLE_BRANCH'],
    headers: {
        'Accept': 'application/vnd.github.v3+json',
        'Authorization': `Basic ${process.env['GITHUB_AUTH']}`
    }
}

const files = [
    // Linux files
    new File('./target/x86_64-unknown-linux-gnu/release/library-loader'),
    new File('./target/x86_64-unknown-linux-gnu/release/library-loader.sha256'),
    // Windows files
    new File('./target/x86_64-pc-windows-gnu/release/library-loader.exe'),
    new File('./target/x86_64-pc-windows-gnu/release/library-loader.exe.sha256')
];

let cargoToml = fs.readFileSync('Cargo.toml', 'utf8').toString().split('\n');
let currentVersion = cargoToml.filter(l => l.includes('version = '))[0].split(' = ')[1].replace(/\"/g, '');

(async () => {

    const releases = await (await fetch(`${gh.url}repos/${gh.user}/${gh.repo}/releases`, {
        headers: gh.headers
    })).json();

    if (semver.lte(currentVersion, releases[0].tag_name)) {
        throw new Error('Current version is not newer than last release!');
    }

    console.log('Creating new release!');

    let newReleaseRes = await (await fetch(`${gh.url}repos/${gh.user}/${gh.repo}/releases`, {
        headers: gh.headers,
        method: 'POST',
        body: JSON.stringify({
            tag_name: currentVersion,
            target_commitish: gh.branch,
            name: `Release v${currentVersion}`,
            body: '',
            draft: false,
            prerelease: false
        })
    })).json();

    let uploadUrl = new URL(newReleaseRes['upload_url'].replace('{?name,label}', ''));

    files.forEach(async file => {

        let baseName = path.basename(file.path);
        uploadUrl.searchParams.set('name', baseName);
        uploadUrl.searchParams.set('label', baseName);

        let res = await fetch(uploadUrl, {
            headers: {
                ...gh.headers,
                'Content-Type': file.mime,
            },
            method: 'POST',
            body: file.read()
        });

        if (await res.status === 201) {
            console.log(`Uploaded file ${baseName}`)
        } else {
            console.error(`Failed to upload ${baseName}`)
        }

    });

})();


