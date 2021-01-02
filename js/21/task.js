// const rawData = require('fs').readFileSync('testinput', 'UTF-8').split('\n');
const rawData = require('fs').readFileSync('input', 'UTF-8').split('\n');
rawData.pop();

function parseLine(line) {
    const [, lhs, rhs] = /^([^\(]+) \(contains (.+)\)$/.exec(line);
    return [new Set(lhs.split(' ')), new Set(rhs.split(', '))];
}
const data = rawData.map(parseLine);
const unmatchedAllergens = new Set(data.map(([, a]) => Array.from(a)).flat());

function intersection(...sets) {
    return sets.reduce((r, s) => new Set([...r].filter((v) => s.has(v))), sets[0]);
}

function removeProductAllergen(thisData, product, allergen) {
    for (const row of thisData) {
        row[0].delete(product);
        row[1].delete(allergen);
    }
}
console.time("Part 1");
(() => {
    const thisData = [...data];
    const allergenMap = new Map();
    while (unmatchedAllergens.size !== 0) {
        unmatchedAllergens.forEach((allergen) => {
            const affectedProducts = thisData.filter(([, v]) => v.has(allergen)).map(([v]) => v);
            const common = intersection(...affectedProducts);
            if (common.size === 1) {
                const [product] = [...common];
                removeProductAllergen(thisData, product, allergen);
                unmatchedAllergens.delete(allergen);
                allergenMap.set(allergen, product);
            }
        });
    }
    console.debug(allergenMap);

    console.info(thisData.reduce((acc, [v]) => acc + v.size, 0));
    console.info(Array.from(allergenMap.entries()).sort().map(([, v])=> v).join());
})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
})();
console.timeEnd("Part 2");
