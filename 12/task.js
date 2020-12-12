const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split('\n');
lines.pop();

console.debug({
    first: lines[0],
    last: lines[lines.length - 1],
    length: lines.length
});

let wPos = [10, 1];
let sPos = [0, 0];

function move(pos, dir, num) {
    switch (dir) {
        case 'N':
            return [pos[0], pos[1] + num];
        case 'S':
            return [pos[0], pos[1] - num];
        case 'E':
            return [pos[0] + num, pos[1]];
        case 'W':
            return [pos[0] - num, pos[1]];
    }
}

for (const line of lines) {
    const res = /^(.)(\d+)$/.exec(line);
    const act = res[1];
    const num = Number(res[2]);

    console.debug({act, num, wPos, sPos});
    if (act === 'L' || act === 'R') {
        const times = num / 90 * (act === 'L' ? -1 : +1);
        switch ((4 + times) % 4) {
            case 1:
                wPos = [wPos[1], -wPos[0]];
                break;
            case 2:
                wPos = [-wPos[0], -wPos[1]];
                break;
            case 3:
                wPos = [-wPos[1], wPos[0]];
                break;
        }
    } else if (act === 'F') {
        sPos = [sPos[0] + num * wPos[0], sPos[1] + num * wPos[1]];
    } else {
        wPos = move(wPos, act, num);
    }
}

console.debug({sPos})
console.debug(sPos[0] + sPos[1]);
