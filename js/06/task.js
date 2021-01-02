const data = require('fs').readFileSync('input', 'UTF-8');
const groups = data.split('\n\n');

console.time("part2");
const count = groups.reduce((acc, v) => {
    const lines = v.split('\n').filter((l) => l !== '');
    const c = Array.from('abcdefghijklmnopqrstuvwxyz')
        .filter((ch) => lines.every((line) => line.includes(ch)))
        .length;
    return acc + c;
}, 0);
console.timeEnd("part2");

console.log(count);

