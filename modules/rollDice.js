const help = [
    "**BASIC COMMANDS**",
    "- `!r d20` - roll a 20-sided dice",
    "- `!r 2d6` - roll two six-sided dice",
    "**FLAGS**",
    "- `-s` - show stats (number of crits and botches)",
    "- `-e N` - explode on given number or max roll if no number is given (keep rolling and add results together)"
];

const rollDice = args => {
    if (args[0] === "help") {
        return help;
    }
    const [numDice, sides] = args[0].toLowerCase().split("d").map(Number);
    const showStats = args.indexOf("-s") !== -1;

    const explode = args.indexOf("-e") !== -1;
    const explodeOn = explode
        ? Number(args[args.indexOf("-e") + 1]) || sides
        : null;

    if (isNaN(numDice) || isNaN(sides)) {
        return "Invalid syntax. Use `dX` to roll 1 X-sided die or `YdX` to roll Y X-sided dice. Use `help` for more info.";
    }

    const results = new Array(numDice || 1).fill(0);
    for (let i = 0; i !== results.length; ++i) {
        const roll = Math.floor(Math.random() * sides + 1);
        results[i] += roll;
        if (explode && roll === explodeOn) {
            --i;
            continue;
        }
    }
    return [
        results.map(n => (n >= sides ? `\`[${n}]\`` : `\`${n}\``)).join(" "),
        showStats ? `Crits: ${results.filter(n => n >= sides).length}` : null,
        showStats ? `Botches: ${results.filter(n => n === 1).length}` : null
    ];
};

export default rollDice;
