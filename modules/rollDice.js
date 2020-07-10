const rollDice = (args, showStats) => {
    const [numDice, sides] = args.toLowerCase().split("d").map(Number);
    if (isNaN(numDice) || isNaN(sides)) {
        return "Invalid syntax. Use dX to roll 1 X-sided die or YdX to roll Y X-sided dice.";
    }
    const results = new Array(numDice || 1);
    for (let i = 0; i !== results.length; ++i) {
        results[i] = Math.floor(Math.random() * sides + 1);
    }
    return [
        results.map(n => (n === sides ? `\`[${n}]\`` : `\`${n}\``)).join(" "),
        showStats
            ? `Crits: ${results.reduce(
                  (acc, v) => (v === sides ? acc + 1 : acc),
                  0
              )}`
            : null,
        showStats
            ? `Botches: ${results.reduce(
                  (acc, v) => (v === 1 ? acc + 1 : acc),
                  0
              )}`
            : null
    ];
};

export default rollDice;
