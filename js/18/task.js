const data = require('fs').readFileSync('input', 'UTF-8');
const rawData = data.split('\n');
rawData.pop();

console.time("Part 1");
(() => {
    function parseLine(s) {
        const acc = [];
        for (const ch of s) {
            if (ch === ' ') continue;
            else if (Number(ch)) {
                const prev = acc[acc.length - 1];
                if (prev === '+') {
                    acc.pop();
                    const v = acc.pop();
                    acc.push(v + Number(ch));
                } else if (prev === '*') {
                    acc.pop();
                    const v = acc.pop();
                    acc.push(v * Number(ch));
                } else acc.push(Number(ch));
            } else if (ch === '(') {
                acc.push('(');
            } else if (ch === ')') {
                const val = acc.pop();
                acc.pop();

                const prev = acc[acc.length - 1];
                if (prev === '+') {
                    acc.pop();
                    const v = acc.pop();
                    acc.push(v + val);
                } else if (prev === '*') {
                    acc.pop();
                    const v = acc.pop();
                    acc.push(v * val);
                } else acc.push(val);

            } else acc.push(ch);
        }
        return acc;
    }

    const results = rawData.map((v) => parseLine(v));
    console.info(results.reduce((acc, [v]) => acc + v, 0 ));
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
    function eval(acc) {
        let ans = 1;
        while (acc.length) {
            const prev = acc.pop();
            if (prev === '(') {
                acc.push(ans);
                return;
            }
            if (prev === '*') {
                ans *= acc.pop();
            } else { ans *= prev; };
        }
        acc.push(ans);
    }

    function parseLine(s) {
        const acc = [];
        for (const ch of s) {
            if (ch === ' ') continue;
            else if (Number(ch)) {
                const prev = acc[acc.length - 1];
                if (prev === '+') {
                    acc.pop();
                    const v = acc.pop();
                    acc.push(v + Number(ch));
                } else acc.push(Number(ch));
            } else if (ch === '(') {
                acc.push('(');
            } else if (ch === ')') {
                eval(acc);

                const prev = acc[acc.length - 1];
                const prev2 = acc[acc.length - 2];
                if (prev2 === '+') {
                    acc.pop();
                    acc.pop();
                    acc.push(acc.pop() + prev);
                }
            } else acc.push(ch);
        }
        eval(acc);
        return acc;
    }

    const results = rawData.map((v) => parseLine(v));
    console.debug(results);
    console.info(results.reduce((acc, [v]) => acc + v, 0 ));
})();
console.timeEnd("Part 2");
