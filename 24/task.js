// const rawData = require('fs').readFileSync('testinput', 'UTF-8').split('\n');
const rawData = require('fs').readFileSync('input', 'UTF-8').split('\n');
rawData.pop();

function parseLine(line) {
    let north = 0;
    let east = 0;
    while (line.length) {
        const dir = line.shift();
        if (dir === 'n' || dir === 's') {
            north += (dir === 'n') ? 2 : -2;
            switch (line.shift()) {
                case 'e': { east += 1; break; }
                case 'w': { east -= 1; break; }
            }
        } else {
            east += (dir === 'e') ? 2 : -2;
        }
    }

    return [north, east];
}

console.time("Part 1");
(() => {
    const flipped = new Map();
    for (const line of rawData) {
        const id = parseLine(line.split('')).join();
        flipped.set(id, !flipped.get(id));
    }

    console.info(
        Array.from(flipped.entries()).reduce((acc, [, v]) => acc + (v ? 1 : 0), 0)
    );
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
    let flipped = new Map();
    let mine = 999;
    let maxe = -999;
    let minn = 999;
    let maxn = -999;
    for (const line of rawData) {
        const [n, e] = parseLine(line.split(''));
        if (n > maxn) { maxn = n; }
        if (n < minn) { minn = n; }

        if (e > maxe) { maxe = e; }
        if (e < mine) { mine = e; }

        flipped.set([n, e].join(), !flipped.get([n, e].join()));
    }

    function neighbors(map, n, e) {
        return !!map.get([n, e + 2].join()) + !!map.get([n, e - 2].join()) +
            !!map.get([n + 2, e + 1].join()) + !!map.get([n + 2, e - 1].join()) +
            !!map.get([n - 2, e + 1].join()) + !!map.get([n - 2, e-1].join());
    }

    function evolve(map, [[minn, maxn], [mine, maxe]]) {
        let _minn = minn + 100;
        let _maxn = maxn - 100;
        let _mine = mine + 100;
        let _maxe = maxe - 100;

        const result = new Map();
        for (let n = minn - 2; n <= maxn + 2; n += 2) {
            for (let e = mine - 2; e <= maxe + 2; ++e) {
                const count = neighbors(map, n, e);
                let r;
                if (map.get([n, e].join())) {
                    r = (count === 1 || count === 2);
                } else {
                    r = (count === 2);
                }
                result.set([n, e].join(), r);
                if (r) {
                    if (n > _maxn) { _maxn = n; }
                    if (n < _minn) { _minn = n; }

                    if (e > _maxe) { _maxe = e; }
                    if (e < _mine) { _mine = e; }
                }
            }
        }

        return [result, [[_minn, _maxn], [_mine, _maxe]]];
    }

    for (let i = 0; i < 100; ++i) {
        [flipped, [[minn, maxn], [mine, maxe]]] = evolve(flipped, [[minn, maxn], [mine, maxe]]);
    }
    console.info(Array.from(flipped.entries()).reduce((acc, [, v]) => acc + (v ? 1 : 0), 0)
    );
})();
console.timeEnd("Part 2");
