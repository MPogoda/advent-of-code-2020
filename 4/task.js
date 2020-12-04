const fs = require('fs');

const data = fs.readFileSync('input', 'UTF-8');
const lines = data.split(/r?\n/);

console.debug(lines);

const fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'];

const set = new Set();

let count = 0;

for (const line of lines) {
    if (line === '') {
        count += fields.every((field) => set.has(field)) ? 1 : 0;
        set.clear();
    }
    for (const entry of line.split(' ')) {
        const [name, value] = entry.split(':');
        switch (name) {
            case 'byr':
                if (value.length !== 4 || Number(value) < 1920 || Number(value) > 2002) continue;
                break;
            case 'iyr':
                if (value.length !== 4 || Number(value) < 2010 || Number(value) > 2020) continue;
                break;
            case 'eyr':
                if (value.length !== 4 || Number(value) < 2020 || Number(value) > 2030) continue;
                break;
            case 'hgt':
                const parsed = /^(\d+)(in|cm)$/.exec(value);
                if (!parsed) continue;
                switch (parsed[2]) {
                    case 'cm':
                        if (Number(parsed[1]) < 150 || Number(parsed[1]) > 193) continue;
                        break;
                    case 'in':
                        if (Number(parsed[1]) < 59 || Number(parsed[1]) > 76) continue;
                        break;
                    default:
                        continue;
                }
                break;
            case 'hcl':
                if (!/^#[0-9a-f]{6}$/.exec(value)) continue;
                break;
            case 'ecl':
                if (!['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].includes(value)) continue;
                break;
            case 'pid':
                if (!/^\d{9}$/.exec(value)) continue;
                break;
            default: continue;
        }
        set.add(name);
    }
}

console.log(count);
