const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split('\n');
lines.pop();

console.debug({
    first: lines[0],
    last: lines[lines.length - 1],
    length: lines.length
});

const numbers = lines.map((v) => Number(v)).sort((a, b) => a - b);
console.debug(numbers);

// let ans1 = 0;
// let ans3 = 1;
// let prev = 0;
//
const set = new Set(numbers);
const target = numbers[numbers.length - 1];

// for (const n of numbers) {
//     if (n - prev === 1) ++ans1;
//     if (n - prev === 3) ++ans3;
//     prev = n;
// }

const visited = new Map();
function traverse(current) {
    if (visited.has(current)) return visited.get(current);

    if (current === target) return 1;
    const ans = [1,2,3].map((inc) => {
        if (set.has(current + inc)) {
            return traverse(current+inc);
        }
        return 0;
    }).reduce((acc, v) => acc+v, 0);
    visited.set(current, ans);
    return ans;
}

console.debug(traverse(0));
