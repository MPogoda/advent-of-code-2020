function part1() {
    const input = [12,20,0,6,1,17,7];
    // const input = [0,3,6];
    const map = new Map(input.map((v, i) => [v, i]));

    let length = input.length;

    while (length < 30000000) {
        const prev = input[length - 1];
        if (!map.has(prev)) {
            input.push(0);
        } else {
            input.push(length - 1 - map.get(prev));
        }
        map.set(prev, length - 1);
        ++length;
    }

    return input[30000000 - 1];
}

console.debug(part1());
