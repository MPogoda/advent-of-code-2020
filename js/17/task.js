const data = require('fs').readFileSync('input', 'UTF-8');
const rawData = data.split('\n').map((r) => r.split(''));
rawData.pop();

const N = rawData.length;

console.time("Part 1");
(() => {
    function neighbours(space, X, Y, Z) {
        let ans = 0;
        for (let z = Z - 1; z <= Z + 1; ++z) {
            if (!space[z]) continue;
            for (let y = Y - 1; y <= Y + 1; ++y) {
                if (!space[z][y]) continue;
                for (let x = X - 1; x <= X + 1; ++x) {
                    ans += space[z][y][x] === '#';
                }
            }
        }
        return ans;
    }

    function evolve(space, Z) {
        const newSpace = []
        let count = 0;
        for (let z = -Z - 1; z <= Z + 1; ++z) {
            const thisLayer = space[z] || [];
            const newLayer = [];
            let wasActiveLayer = false;

            for (let y = -Z - 1; y < N + Z + 1; ++y) {
                const thisRow = thisLayer[y] || [];
                const newRow = [];
                let wasActiveRow = false;
                for (let x = -Z - 1; x < N + Z + 1; ++x) {
                    const thisE = thisRow[x] || '.';
                    const thisNeigbours = neighbours(space, x, y, z);
                    if (thisE === '.') {
                        newRow[x] = thisNeigbours === 3 ? '#' : '.';
                    } else {
                        newRow[x] = [4, 3].includes(thisNeigbours) ? '#' : '.';
                    }
                    wasActiveRow = wasActiveRow || newRow[x] === '#';
                    count += newRow[x] === '#';
                }
                newLayer[y] = [];
                if (wasActiveRow) {
                    newLayer[y] = newRow;
                    wasActiveLayer = true;
                }
            }
            if (wasActiveLayer) newSpace[z] = newLayer;
        }
        console.debug("Iteration", Z + 1, count);
        return newSpace;
    }

    let s = [rawData];
    for (let z = 0; z < 6; ++z) {
        s = evolve(s, z);
    }
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
    function neighbours(space, X, Y, Z, W) {
        let ans = 0;
        for (let w = W - 1; w <= W + 1; ++w) {
            if (!space[w]) continue;
            for (let z = Z - 1; z <= Z + 1; ++z) {
                if (!space[w][z]) continue;
                for (let y = Y - 1; y <= Y + 1; ++y) {
                    if (!space[w][z][y]) continue;
                    for (let x = X - 1; x <= X + 1; ++x) {
                        ans += space[w][z][y][x] === '#';
                    }
                }
            }
        }
        return ans;
    }

    function evolve(space, Z) {
        const newSpace = [];
        let count = 0;

        for (let w = -Z - 1; w <= Z + 1; ++w) {
            const thisCube = space[w] || [];
            const newCube = [];
            for (let z = -Z - 1; z <= Z + 1; ++z) {
                const thisLayer = thisCube[z] || [];
                const newLayer = [];
                let wasActiveLayer = false;

                for (let y = -Z - 1; y < N + Z + 1; ++y) {
                    const thisRow = thisLayer[y] || [];
                    const newRow = [];
                    let wasActiveRow = false;
                    for (let x = -Z - 1; x < N + Z + 1; ++x) {
                        const thisE = thisRow[x] || '.';
                        const thisNeigbours = neighbours(space, x, y, z, w);
                        if (thisE === '.') {
                            newRow[x] = thisNeigbours === 3 ? '#' : '.';
                        } else {
                            newRow[x] = [4, 3].includes(thisNeigbours) ? '#' : '.';
                        }
                        wasActiveRow = wasActiveRow || newRow[x] === '#';
                        count += newRow[x] === '#';
                    }
                    newLayer[y] = [];
                    if (wasActiveRow) {
                        newLayer[y] = newRow;
                        wasActiveLayer = true;
                    }
                }
                if (wasActiveLayer) newCube[z] = newLayer;
            }
            newSpace[w] = newCube;
        }
        console.debug("Iteration", Z + 1, count);
        return newSpace;
    }

    let s = [[rawData]];
    for (let z = 0; z < 6; ++z) {
        s = evolve(s, z);
    }
})();
console.timeEnd("Part 2");
