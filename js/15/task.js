console.time("hm");
function part1() {
    const input = [12,20,0,6,1,17,7];
    // const input = [0,3,6];
    const map = new Array(30000000);
    input.forEach((v, i) => map[v] = i);

    let length = input.length;

    while (length < 30000000) {
        const prev = input[length - 1];
        if (map[prev] === undefined) {
            input.push(0);
        } else {
            input.push(length - 1 - map[prev]);
        }
        map[prev] = length - 1;
        ++length;
    }

    return input[30000000 - 1];
}

console.debug(part1());
console.timeEnd("hm");
