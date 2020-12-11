const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split('\n').map((r) => r.split(''));
lines.pop();

console.debug({
    first: lines[0],
    last: lines[lines.length - 1],
    length: lines.length
});

const N = lines.length;
const M = lines[0].length;

function occupied(input, I, J) {
    let count = 0;
    for (let i = I - 1, j = J; i >= 0; --i) {
        if (input[i][j] === 'L') break;
        if (input[i][j] === '#') {
            ++count;
            break;
        }
    }

    for (let i = I - 1, j = J + 1; i >= 0 && j < M; --i, ++j) {
        if (input[i][j] === 'L') break;
        if (input[i][j] === '#') {
            ++count;
            break;
        }
    }

    for (let i = I, j = J + 1; i >= 0 && j < M; i, ++j) {
        if (input[i][j] === 'L') break;
        if (input[i][j] === '#') {
            ++count;
            break;
        }
    }

    for (let i = I + 1, j = J + 1; i < N && j < M; i++, ++j) {
        if (input[i][j] === 'L') break;
        if (input[i][j] === '#') {
            ++count;
            break;
        }
    }

    for (let i = I + 1, j = J; i < N && j < M; i++, j) {
        if (input[i][j] === 'L') break;
        if (input[i][j] === '#') {
            ++count;
            break;
        }
    }

    for (let i = I + 1, j = J - 1; i < N && j >=0; i++, --j) {
        if (input[i][j] === 'L') break;
        if (input[i][j] === '#') {
            ++count;
            break;
        }
    }

    for (let i = I, j = J - 1; i < N && j >=0; i, --j) {
        if (input[i][j] === 'L') break;
        if (input[i][j] === '#') {
            ++count;
            break;
        }
    }

    for (let i = I - 1, j = J - 1; i >= 0 && j >=0; --i, --j) {
        if (input[i][j] === 'L') break;
        if (input[i][j] === '#') {
            ++count;
            break;
        }
    }

    return count;
}

function iteration(input) {
    const result = [];
    let wasChange = false;
    for (let i = 0; i < N; ++i) {
        const row = [];
        for (let j = 0; j < M; ++j) {
            if (input[i][j] === '.') {
                row.push('.');
            } else if (input[i][j] === 'L') {
                if (occupied(input, i, j)) {
                    row.push('L');
                } else {
                    row.push('#');
                    wasChange = true;
                }
            } else if (input[i][j] === '#') {
                if (occupied(input, i, j) < 5) {
                    row.push('#');
                } else {
                    row.push('L');
                    wasChange = true;
                }
            }
        }
        result.push(row);
    }
    return [result, wasChange];
}

let plan = lines;
let wasChange = true;

while (wasChange) {
    [plan, wasChange] = iteration(plan);
    console.debug({wasChange, prev: plan[95].join(''), last: plan[96].join('')});
}

console.log(plan.reduce((acc, row) => row.reduce(
    (accr, v) => accr + (v === '#' ? 1 : 0),
    acc
), 0));
