const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split('\n');
lines.pop();

console.debug({
    first: lines[0],
    last: lines[lines.length - 1],
    length: lines.length
});

function part1() {
    const mem = {};
    for (const line of lines) {
        const maskRes = /^mask = (.+)$/.exec(line);
        if (maskRes) {
            mask = maskRes[1].split('').reverse();
            continue;
        }
        const [, addr, int] = /^mem\[(\d+)\] = (\d+)$/.exec(line);
        console.debug({addr, int});
        const num = Number(int).toString(2).split('').reverse();

        const newNum = mask.map((m, i) => {
            if (m !== 'X') return m;
            return i < num.length ? num[i] : '0';
        });
        mem[addr] = parseInt(newNum.reverse().join(''), 2);
    }

    console.debug(Object.values(mem).reduce((acc, v) => acc + v, 0));
}

function getAddresses(mm, addr) {
    const convertedAddr = Number(addr).toString(2).split('').reverse();
    const appliedMask = mm.map((m, i) => {
        if (m !== '0') return m;
        return i < convertedAddr.length ? convertedAddr[i] : '0';
    });

    const res = traverse(appliedMask, 0);
    return res;
}

function traverse(mm, start) {
    for (let i = start; i < mm.length; ++i) {
        if (mm[i] !== 'X') continue;
        mm[i] = '0';
        const res1 = traverse(mm, i + 1);
        mm[i] = '1';
        const res2 = traverse(mm, i + 1);
        mm[i] = 'X';
        return [...res1, ...res2];
    }
    return [parseInt([...mm].reverse().join(''), 2)];
}

let mask = [];
const mem = {};
for (const line of lines) {
    const maskRes = /^mask = (.+)$/.exec(line);
    if (maskRes) {
        mask = maskRes[1].split('').reverse();
        continue;
    }
    const [, addr, int] = /^mem\[(\d+)\] = (\d+)$/.exec(line);
    const num = Number(int);

    for (const a of getAddresses(mask, addr)) {
        mem[a] = num;
    }
}

console.debug(Object.values(mem).reduce((acc, v) => acc + v, 0));
