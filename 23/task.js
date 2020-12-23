// const rawData = require('fs').readFileSync('testinput', 'UTF-8').split('\n\n');
const rawData = require('fs').readFileSync('input', 'UTF-8').split('').map(Number);
rawData.pop();

console.debug(rawData);

console.time("Part 1");
(() => {
    const arr = rawData.slice();
    function move() {
        const current = arr.shift();
        const cups = arr.splice(0, 3);

        let target = current - 1;
        while (true) {
            if (target === 0) target = 9;
            if (!cups.includes(target)) {
                break;
            }
            --target;
        }

        let index = arr.findIndex((x) => x === target);
        arr.splice(index + 1, 0, ...cups);
        arr.push(current);
    }

    for (let i = 0; i < 100; ++i) move();

    const idx = arr.findIndex((x) => x === 1);
    console.info([...arr, ...arr].slice(idx + 1, idx + arr.length).join(''));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
    const map = new Array(1000000 + 1).fill(1).map((_, i) => i + 1);
    map[1000000] = rawData[0];
    for (let i = 0; i < rawData.length - 1; ++i) {
        map[rawData[i]] = rawData[i + 1];
    }
    map[rawData[rawData.length - 1]] = rawData.length + 1;

    let current = rawData[0];
    for (let i = 0; i < 10000000; ++i) {
        let c0 = map[current];
        let c1 = map[c0];
        let c2 = map[c1];
        let target = current > 1 ? current - 1 : 1000000;
        while ([c0, c1, c2].includes(target)) {
            target = target > 1 ? target - 1 : 1000000;
        }

        map[current] = map[c2];
        current = map[current];

        const t = map[target];
        map[target] = c0;
        map[c2] = t;
    }

    console.info(map[1] * map[map[1]]);
})();
console.timeEnd("Part 2");
