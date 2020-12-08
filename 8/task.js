const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split('\n');
lines.pop();

function tryProgram(program) {
    const set = new Set();
    let acc = 0;
    let ip = 0;

    while (!set.has(ip) && ip < program.length) {
        set.add(ip);
        const [instr, param] = program[ip].split(' ');
        switch (instr) {
            case 'acc': {
                acc += Number(param);
                ip++;
                break;
            }
            case 'jmp': {
                ip += Number(param);
                break;
            }
            case 'nop': {
                ip++;
                break;
            }
        }
    }

    if (program.length === ip) {
        return acc;
    }
    return undefined;
}

for (let i = 0; i < lines.length; ++i) {
    const [instr, param] = lines[i].split(' ');
    if (instr === 'jmp') {
        const maybe = tryProgram([
            ...lines.slice(0, i),
            'nop 0',
            ...lines.slice(i + 1)
        ]);
        if (maybe != null) {
            console.debug(maybe);
            return;
        }
    }
    if (instr === 'nop') {
        const maybe = tryProgram([
            ...lines.slice(0, i),
            `jmp ${param}`,
            ...lines.slice(i + 1)
        ]);
        if (maybe != null) {
            console.debug(maybe);
            return;
        }
    }
}

