const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split('\n');
lines.pop();

console.debug({
    first: lines[0],
    last: lines[lines.length - 1],
    length: lines.length
});

const busses = lines[1].split(',').map((x) => Number(x));

let t = 0;
let step = 1;

busses.forEach((bus, i) => {
    if (isNaN(bus)) return;
    const want = (100*bus - i) % bus;
    console.debug({want, bus});
    while (t % bus !== want) t += step;
    step *= bus;
    console.log({t, step});
});

console.log(t);


