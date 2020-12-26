const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const rawData = data.split('\n\n');

console.time("Input");
const rawRules = rawData[0].split('\n');
const thisTicket = rawData[1].split('\n')[1].split(',').map((v) => Number(v));
const otherTickets = rawData[2].split('\n').slice(1, -1).map(
    (v) => v.split(',').map((x) => Number(x))
);

const rulesMatch = new Array(1000).fill(1).map(() => new Set());

const allNames = [];
for (const rule of rawRules) {
    const [, name, r1s, r1e, r2s, r2e] = /^([^\d]+): (\d+)-(\d+) or (\d+)-(\d+)$/.exec(rule);
    allNames.push(name);
    for (let i = Number(r1s); i <= Number(r1e); ++i) rulesMatch[i].add(name);
    for (let i = Number(r2s); i <= Number(r2e); ++i) rulesMatch[i].add(name);
}
console.timeEnd("Input");

console.time("Part 1");
console.info(otherTickets.reduce(
    (ans, ticket) => ans + ticket.reduce(
        (acc, v) => acc + (rulesMatch[v].size ? 0 : v), 0
    ), 0
));
console.timeEnd("Part 1");

const validTickets = otherTickets.filter((vs) => vs.every(
    (v) => rulesMatch[v].size
));

const possibleNames = thisTicket.map(() => ([...allNames]));

function cleanNonAmbiguous(matchedName) {
    const toRemove = [];

    for (const i in possibleNames) {
        if (possibleNames[i].length <= 1) continue;
        possibleNames[i] = possibleNames[i].filter((s) => s !== matchedName);
        if (possibleNames[i].length === 1) toRemove.push(possibleNames[i][0]);
    }

    for (const name of toRemove) cleanNonAmbiguous(name);
}

console.time("Part 2");
for (const ks of validTickets) {
    ks.forEach((k, i) => {
        if (possibleNames[i].length <= 1) return;
        possibleNames[i] = possibleNames[i].filter((s) => rulesMatch[k].has(s));
        if (possibleNames[i].length === 1) {
            cleanNonAmbiguous(possibleNames[i][0]);
        }
    });
}

console.info(
    possibleNames.reduce((ans, [name], i) => ans * (name.startsWith("departure") ? thisTicket[i] : 1), 1)
);
console.timeEnd("Part 2");
