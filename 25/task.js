const rawData = require('fs').readFileSync('input', 'UTF-8').split('\n');
rawData.pop();
const [keyPub, doorPub] = rawData.map(Number);

console.debug(keyPub, doorPub);

console.time("Part 1");
(() => {
    let c = 7;
    let cnt = 1;
    let keyPriv;
    let doorPriv;
    while (!keyPriv || !doorPriv) {
        c *= 7;
        c %= 20201227;
        ++cnt;
        if (c === keyPub) { keyPriv = cnt; console.info({keyPriv}); }
        if (c === doorPub) { doorPriv = cnt; console.info({doorPriv}); }
    }

    const mult = keyPub;
    cnt = doorPriv;
    let ans = 1
    while (cnt > 0) {
        ans *= mult;
        ans %= 20201227;
        --cnt;
    }
    console.info(ans);

})();
console.timeEnd("Part 1");
