// const data = require('fs').readFileSync('testinput', 'UTF-8');
const data = require('fs').readFileSync('input', 'UTF-8');
const rawData = data.split('\n\n');
rawData.pop();

function parseTile(rawLines) {
    const lines = rawLines.split('\n');
    const id = /^Tile (\d+):$/.exec(lines.shift())[1]
    const field = lines.map((l) => l.split('').map((c) => c === '.' ? 0 : 1));

    return [Number(id), field];
}

const N = 12;
// const N = 3;
//
const W = 10;

const tiles = rawData.map((v) => parseTile(v));

function cut(tile) {
    return tile.slice(1, -1).map((row) => row.slice(1, -1));
}

function rotate1(tile) {
    const result = new Array(tile.length).fill(0).map(() => new Array(tile.length).fill(0));

    for (let i = 0; i < tile.length; ++i) {
        for (let j = 0; j < tile.length; ++j) {
            result[i][j] = tile[j][tile.length - i - 1]
        }
    }
    return result;
}

function rotate2(tile) {
    const result = new Array(tile.length).fill(0).map(() => new Array(tile.length).fill(0));

    for (let i = 0; i < tile.length; ++i) {
        for (let j = 0; j < tile.length; ++j) {
            result[i][j] = tile[tile.length - i - 1][tile.length - j - 1]
        }
    }
    return result;
}

function rotate3(tile) {
    const result = new Array(tile.length).fill(0).map(() => new Array(tile.length).fill(0));

    for (let i = 0; i < tile.length; ++i) {
        for (let j = 0; j < tile.length; ++j) {
            result[i][j] = tile[tile.length - j - 1][i]
        }
    }
    return result;
}

const image = new Array(N * (W - 2)).fill(-1).map(() => new Array(N * (W - 2)).fill(-1));
function reconstruct(visited, constructed) {
    const tilesMap = new Map(tiles);
    let x = 0;
    let y = 0;
    for (const id of constructed) {
        let tile;
        switch (Number(visited.get(id)[0])) {
            case 0: {
                tile = cut(tilesMap.get(id));
                break;
            }
            case 1: {
                tile = rotate1(cut(tilesMap.get(id)));
                break;
            }
            case 2: {
                tile = rotate2(cut(tilesMap.get(id)));
                break;
            }
            case 3: {
                tile = rotate3(cut(tilesMap.get(id)));
                break;
            }
            case 4: {
                tile = cut(tilesMap.get(id).slice().reverse());
                break;
            }
            case 5: {
                tile = cut(tilesMap.get(id).map((row) => row.slice().reverse()));
                break;
            }
            case 6: {
                tile = cut(tilesMap.get(id).map((row) => row.slice().reverse()).reverse());
                break;
            }
            case 7: {
                tile = cut(rotate1(tilesMap.get(id)).map(row => row.slice().reverse()));
                break;
            }
            default: {
                console.info("WTF", id, visited.get(id));
            }
        }
        for (let i = 0; i < W - 2; ++i) {
            for (let j = 0; j < W - 2; ++j) {
                image[y + j][x + i] = tile[j][i];
            }
        }

        x += W - 2;
        if (x === N * (W - 2)) {
            x = 0;
            y += W - 2;
        }
    }
}

console.time("Part 1");
(() => {
    function rotates([t, r, b, l]) {
        const rr = [...r].reverse();
        const lr = [...l].reverse();
        const tr = [...t].reverse();
        const br = [...b].reverse();
        return [
            [t, r, b, l],
            [r, br, l, tr],
            [br, lr, tr, rr],
            [lr, t, rr, b],
        ];
    }

    function flipH([t, r, b, l]) {
        const rr = [...r].reverse();
        const lr = [...l].reverse();

        return [b, rr, t, lr];
    }

    function flipV([t, r, b, l]) {
        const tr = [...t].reverse();
        const br = [...b].reverse();

        return [tr, l, br, r];
    }

    function hash([id, field]) {
        const top = field[0];
        const right = field.map((v) => v[v.length - 1]);
        const bottom = field[field.length - 1];
        const left = field.map((v) => v[0]);

        const edges = [top, right, bottom, left];

        const combinations = [
            ...rotates(edges),
            flipH(edges),
            flipV(edges),
            flipH(flipV(edges)),
            flipV(rotates(edges)[1])
        ];

        const hashes = combinations.map((edges) => edges.map((e) => parseInt(e.join(''), 2)));
        return [id, hashes];
    }

    const hashes = tiles.map(hash);
    const hashMap = new Map(hashes);

    const tops = new Map();
    const lefts = new Map();

    for (const [id, hash] of hashes) {
        hash.forEach((nums, rotation) => {
            if (!tops.has(nums[0])) {
                tops.set(nums[0], []);
            }
            tops.get(nums[0]).push([id, rotation]);
            if (!lefts.has(nums[3])) {
                lefts.set(nums[3], []);
            }
            lefts.get(nums[3]).push([id, rotation]);
        });
    }

    const visited = new Map();
    const constructed = [];

    let log = false;

    function recurse(x0, y0) {
        const x = x0 === (N - 1) ? 0 : (x0 + 1);
        const y = x0 === (N - 1) ? (y0 + 1) : y0;

        if (x === 0 && y === N) return true;

        const topHash = y === 0 ? null : visited.get(constructed[(y - 1) * N + x])[1][2];
        const leftHash = x === 0 ? null : visited.get(constructed[y * N + x - 1])[1][1];

        const top = topHash === null ? null : (tops.get(topHash) || []);
        const left = leftHash === null ? null : (lefts.get(leftHash) || []);

        let candidatesTop = null;
        if (top !== null) {
            candidatesTop = top.filter(([id]) => !visited.has(id));
        }
        let candidatesLeft = null;
        if (left !== null) {
            candidatesLeft = left.filter(([id]) => !visited.has(id));
        }
        let candidates = candidatesLeft || candidatesTop;
        if (candidatesLeft !== null && candidatesTop !== null) {
            candidates = candidatesLeft.filter(([id, rot]) => candidatesTop.find(
                ([idT, rotT]) => id === idT && rot === rotT
            ) !== undefined);
        }

        for (const [id, rot] of candidates) {
            constructed.push(id);
            visited.set(id, [rot, hashMap.get(id)[rot]]);
            if (recurse(x, y)) {
                return true;
            }
            visited.delete(id);
            constructed.pop();
        }
        return false;
    }

    for (const [id, hash] of hashes) {
        constructed.push(id);

        for (const rotation in hash) {
            visited.set(id, [rotation, hash[rotation]]);
            if (recurse(0, 0)) {
                console.debug("FOUND", constructed[0] * constructed[11] * constructed[143]* constructed[11*12+0]);
                reconstruct(visited, constructed);
                return;
            }
            visited.delete(id);
        }
        constructed.pop();
    }
})();
console.timeEnd("Part 1");

console.debug(image.map((row) => row.join('')).join('\n'));

console.time("Part 2");
(() => {
    const thisImage = rotate1(image.map((row) => row.reverse()));

    function markMonster(x, y) {
        const L = N * (W - 2);
        if (x + 20 > L) return false;
        if (y + 3 > L) return false;
        const positions = [
            [18, 0],
            [0, 1], [5, 1], [6, 1], [11, 1], [12, 1], [17, 1], [18, 1], [19, 1],
            [1, 2], [4, 2], [7, 2], [10, 2], [13, 2], [16, 2]
        ];
        if (positions.every(([i, j]) => thisImage[y + j][x + i] === 1)) {
            positions.forEach(([i, j]) => thisImage[j + y][i + x] = 2);
            return true;
        }
        return false;
    }

    for (let y = 0; y < N * (W - 2); ++y) {
        for (let x = 0; x < N * (W - 2); ++x) {
            if (markMonster(x, y)) {
                console.debug("Gotcha!");
            }
        }
    }
    let count = 0;

    for (let y = 0; y < N * (W - 2); ++y) {
        for (let x = 0; x < N * (W - 2); ++x) {
            count += thisImage[y][x] === 1;
        }
    }

    console.debug(count);
})();
console.timeEnd("Part 2");
