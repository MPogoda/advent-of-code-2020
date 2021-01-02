// const rawData = require('fs').readFileSync('testinput', 'UTF-8').split('\n\n');
const rawData = require('fs').readFileSync('input', 'UTF-8').split('\n\n');

function parsePlayer(rawLines) {
    const lines = rawLines.split('\n');
    lines.shift();
    return lines.reverse().map(Number);
}

console.time("Part 1");
(() => {
    const player1 = parsePlayer(rawData[0]);
    const player2 = parsePlayer(rawData[1]);

    function playRound() {
        const p1 = player1.pop();
        const p2 = player2.pop();
        if (p1 > p2) {
            player1.unshift(p2, p1);
        } else {
            player2.unshift(p1, p2);
        }
    }
    while (player1.length && player2.length) playRound();
    const winningPlayer = player1.length ? player1 : player2;

    console.info(winningPlayer.reduce((acc, v, i) => acc + v * (i + 1), 0));

})();
console.timeEnd("Part 1");

console.time("Part 2");
(() => {
    const player1 = parsePlayer(rawData[0]);
    const player2 = parsePlayer(rawData[1]);

    function playRound(p1, p2, memo) {
        if (memo.has(`${p1.join()}|${p2.join()}`)) return true;
        memo.add(`${p1.join()}|${p2.join()}`);

        const card1 = p1.pop();
        const card2 = p2.pop();

        if (card1 <= p1.length && card2 <= p2.length) {
            // console.debug("Recursing with", p1.slice(-card1), p2.slice(-card2));
            if (playGame(p1.slice(-card1), p2.slice(-card2))) {
                p1.unshift(card2, card1);
            } else {
                p2.unshift(card1, card2);
            }
        } else {
            // console.debug("Simple", {card1, card2});
            if (card1 > card2) {
                p1.unshift(card2, card1);
            } else {
                p2.unshift(card1, card2);
            }
        }
    }
    function playGame(p1, p2) {
        const memo = new Set();
        while (p1.length && p2.length) {
            if (playRound(p1, p2, memo)) return true;
        }
        return !!p1.length;
    }
    playGame(player1, player2);
    const winningPlayer = player1.length ? player1 : player2;

    console.info(winningPlayer.reduce((acc, v, i) => acc + v * (i + 1), 0));
})();
console.timeEnd("Part 2");
