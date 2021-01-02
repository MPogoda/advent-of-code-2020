const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split('\n');
lines.pop();

console.debug({
    first: lines[0],
    last: lines[lines.length - 1],
    length: lines.length
});

const badNum = 88311122;

let arr=[];
let sum = 0;

for (const v of lines) {
    const num = Number(v);
    sum += num;
    arr.push(num);

    while (sum > badNum && sum != 0) {
        sum -= arr.shift();
    }
    if (sum === badNum && arr.length > 1) {
        console.log({
            arr,
            sum,
            min: Math.min(...arr),
            max: Math.max(...arr)
        });
        return;
    }
}
