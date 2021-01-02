const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split(/r?\n/);

const set = new Set();

for (const line of lines) {
    if (line === '') continue;
    if (line.length != 10) throw Error('wtf' + line.length + line);
    let row = 0;
    for (let t = 0; t <= 6; t++) row += (line[t] === 'B' ? 1 : 0) * (2 ** (6-t));
    let seat = 0;
    for (let t = 7; t <= 9; t++) seat += (line[t] === 'R' ? 1 : 0) * (2 ** (9-t));

    set.add(8 * row + seat);
}

for (let row = 0; row < 128; ++row) {
    for (let seat = 0; seat < 8; ++seat) {
        if (!set.has(8 * row + seat)) {
            console.log({row, seat, id: 8*row+seat});
        }
    }
}
