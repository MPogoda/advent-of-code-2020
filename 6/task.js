const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const groups = data.split('\n\n');

const count = groups.reduce((acc, v) => {
    const lines = v.split('\n').filter((l) => l !== '');
    const c = Array.from('abcdefghijklmnopqrstuvwxyz')
        .filter((ch) => lines.every((line) => line.includes(ch)))
        .length;
    console.debug({lines, c})
    return acc + c;
}, 0);

console.log(count);

