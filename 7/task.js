const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split('\n').sort();

function parseLine(s) {
    if (s === '') return [];
    const [name, rest] = s.split(' bags contain ');
    if (rest === 'no other bags.') return [];
    console.debug({s, name, rest});
    const deps = rest.split(', ').map((v) => {
        const res = /^(\d+) (.+) bag/.exec(v);
        return [res[1], res[2]];
    });
    return [name, deps];
}

const map = {};
for (const line of lines.map(parseLine)) {
    if (line.length === 0) continue;
    const [name, deps] = line;
    map[name] = deps;
}

console.debug(map);

const desiredName = 'shiny gold';

function recurse(name) {
    if (!map[name]) return 1;
    let count = 1;
    for (const [cnt, depName] of map[name]) {
        count += cnt * recurse(depName);
    }
    return count;
}

console.log(recurse(desiredName));
