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
                    if (x === X && y === Y && z === Z) continue;
                    ans += space[z][y][x] === '#';
                }
            }
        }
        return ans;
    }

    function display(space, Z) {
        let ans = 0;
        for (let z = -Z - 1; z <= Z + 1; ++z) {
            if (!space[z]) continue;
            // const acc = ['\n'];
            let count = 0;
            for (let y = -Z - 1; y < N + Z + 1; ++y) {
                if (!space[z][y]) continue;
                for (let x = -Z - 1; x < N + Z + 1; ++x) {
                    // acc.push(space[z][y][x]);
                    if (space[z][y][x] === '#') ++count;
                }
                // acc.push('\n');
            }
            // console.debug(z, count, acc.join(''));
            ans += count;
        }

        console.info("ITERATION ", Z + 1, ans);
    }

    function evolve(space, Z) {
        const newSpace = {}
        for (let z = -Z - 1; z <= Z + 1; ++z) {
            const thisLayer = {...(space[z] || {})};
            const newLayer = {};
            let wasActiveLayer = false;

            for (let y = -Z - 1; y < N + Z + 1; ++y) {
                const thisRow = {...(thisLayer[y] || {})};
                const newRow = {};
                let wasActiveRow = false;
                for (let x = -Z - 1; x < N + Z + 1; ++x) {
                    const thisE = thisRow[x] || '.';
                    const thisNeigbours = neighbours(space, x, y, z);
                    if (thisE === '.') {
                        newRow[x] = thisNeigbours === 3 ? '#' : '.';
                    } else {
                        newRow[x] = [2, 3].includes(thisNeigbours) ? '#' : '.';
                    }
                    wasActiveRow = wasActiveRow || newRow[x] === '#';
                }
                newLayer[y] = {};
                if (wasActiveRow) {
                    newLayer[y] = newRow;
                    wasActiveLayer = true;
                }
            }
            if (wasActiveLayer) newSpace[z] = newLayer;
        }
        display(newSpace, Z);
        return newSpace;
    }

    let s = {0: rawData};
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
                        if (w === W && x === X && y === Y && z === Z) continue;
                        ans += space[w][z][y][x] === '#';
                    }
                }
            }
        }
        return ans;
    }

    function display(space, Z) {
        let ans = 0;
        for (let w = -Z - 1; w <= Z + 1; ++w) {
            if (!space[w]) continue;
            for (let z = -Z - 1; z <= Z + 1; ++z) {
                if (!space[w][z]) continue;
                let count = 0;
                for (let y = -Z - 1; y < N + Z + 1; ++y) {
                    if (!space[w][z][y]) continue;
                    for (let x = -Z - 1; x < N + Z + 1; ++x) {
                        if (space[w][z][y][x] === '#') ++count;
                    }
                }
                ans += count;
            }
        }

        console.info("ITERATION ", Z + 1, ans);
    }

    function evolve(space, Z) {
        const newSpace = {}
        for (let w = -Z - 1; w <= Z + 1; ++w) {
            const thisCube = {...(space[w] || {})};
            const newCube = {};
            for (let z = -Z - 1; z <= Z + 1; ++z) {
                const thisLayer = {...(thisCube[z] || {})};
                const newLayer = {};
                let wasActiveLayer = false;

                for (let y = -Z - 1; y < N + Z + 1; ++y) {
                    const thisRow = {...(thisLayer[y] || {})};
                    const newRow = {};
                    let wasActiveRow = false;
                    for (let x = -Z - 1; x < N + Z + 1; ++x) {
                        const thisE = thisRow[x] || '.';
                        const thisNeigbours = neighbours(space, x, y, z, w);
                        if (thisE === '.') {
                            newRow[x] = thisNeigbours === 3 ? '#' : '.';
                        } else {
                            newRow[x] = [2, 3].includes(thisNeigbours) ? '#' : '.';
                        }
                        wasActiveRow = wasActiveRow || newRow[x] === '#';
                    }
                    newLayer[y] = {};
                    if (wasActiveRow) {
                        newLayer[y] = newRow;
                        wasActiveLayer = true;
                    }
                }
                if (wasActiveLayer) newCube[z] = newLayer;
            }
            newSpace[w] = newCube;
        }
        display(newSpace, Z);
        return newSpace;
    }

    let s = {0: {0: rawData}};
    for (let z = 0; z < 6; ++z) {
        s = evolve(s, z);
    }
})();
console.timeEnd("Part 2");
