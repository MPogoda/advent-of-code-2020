// const data = require('fs').readFileSync('testinput', 'UTF-8');
// const data = require('fs').readFileSync('testinput2', 'UTF-8');
const data = require('fs').readFileSync('input', 'UTF-8');
const rawData = data.split('\n\n');

const rawRules = rawData[0].split('\n');
const messages = rawData[1].split('\n').map((v) => v.split(''));
messages.pop();

const rules = new Map(rawRules.map((rawRule) => {
    const [num, rule] = rawRule.split(': ');
    return [num, rule.split(' | ').map((v) => v.split(' '))];
}));

console.time("Part 1");
(() => {
    function verify(message, ruleN) {
        const rule = rules.get(ruleN);
        let bestMatch = 0;
        for (const subRule of rule) {
            if (subRule.length === 1 && !Number(subRule[0])) {
                if (subRule[0][1] === message[0]) bestMatch = 1;
                continue;
            }
            let wasMatch = true;
            let matchedSoFar = 0;
            for (const p of subRule) {
                const matchedLength = verify(message.slice(matchedSoFar), p);
                if (!matchedLength) {
                    wasMatch = false;
                    break;
                }
                matchedSoFar += matchedLength;
                if (matchedSoFar > message.length) {
                    wasMatch = false;
                    break;
                }
            }
            if (wasMatch) bestMatch = matchedSoFar;
        }
        return bestMatch;
    }
    console.info(messages.filter((m) => m.length === verify(m, '0')).length);
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
    function verify(message, ruleN) {
        const rule = rules.get(ruleN);
        let matches = [];
        for (const subRule of rule) {
            if (subRule.length === 1 && !Number(subRule[0])) {
                if (subRule[0][1] === message[0]) matches.push(1);
                continue;
            }
            // [idx, slice]
            let queue = [[0, 0]];
            while (queue.length) {
                const [p, slice] = queue.shift();
                if (slice > message.length) continue;
                if (p === subRule.length) {
                    matches.push(slice);
                    continue;
                }
                for (const mm of verify(message.slice(slice), subRule[p])) {
                    queue.push([p + 1, slice + mm]);
                }
            }
        }
        return matches;
    }
    rules.set('8', [['42'], ['42', '8']]);
    rules.set('11', [['42', '31'], ['42', '11', '31']]);
    console.info(messages.filter((m) => verify(m, '0').includes(m.length)).length);
})();
console.timeEnd("Part 2");
