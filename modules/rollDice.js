const help = [
    "**BASIC COMMANDS**",
    "`!r d20` - roll a 20-sided dice",
    "`!r 2d6` - roll two six-sided dice",
    "**OPTIONS**",
    "`-tN` - rolls N or higher are marked as successes",
    "`-fN` - rolls N or lower are marked as failures and count against successes",
    "`-eN` - explode on N or max roll if no number is given (keep rolling and add results together)",
    "`-kN` - keep only the best N rolls",
    "`-dN` - drop the lowest N rolls",
    "`-rN,M` - reroll any dice equal or less than N (max. M times, defaults to 1)"
];

const rollDice = args => {
    if (args[0] === "help") {
        return help;
    }
    const [numDice, sides] = args[0].toLowerCase().split("d").map(Number);

    const options = args.slice(1).join(" ");
    const target = options.match(/-t(\d*)/);
    const failures = options.match(/-f(\d*)/);
    const explode = options.match(/-e(\d*)/);
    const keep = options.match(/-k(\d*)/);
    const drop = options.match(/-d(\d*)/);
    const reroll = options.match(/-r(\d*),(\d*)/);

    if (isNaN(numDice) || isNaN(sides)) {
        return "Invalid syntax. Use `dX` to roll 1 X-sided die or `YdX` to roll Y X-sided dice. Use `help` for more info.";
    }

    const rolls = new Array(numDice || 1).fill(0);
    let rerollRounds = reroll ? Number(reroll[2]) || 1 : 0;
    for (let i = 0; i !== rolls.length; ++i) {
        const roll = Math.floor(Math.random() * sides + 1);
        rolls[i] += roll;
        if (explode && roll === Number(explode[1])) {
            --i;
        } else if (reroll && rerollRounds && roll <= Number(reroll[1])) {
            console.log(rerollRounds);
            rolls[i--] = 0;
            --rerollRounds;
            continue;
        }
        rerollRounds = reroll ? Number(reroll[2]) || 1 : 0;
    }

    const sortedRolls = rolls.sort((a, b) => a - b);
    let results;
    if (keep) {
        results = sortedRolls.slice(Number(keep[1]) * -1);
    } else if (drop) {
        results = sortedRolls.slice(Number(drop[1]));
    } else {
        results = rolls;
    }

    const successCnt = target
        ? results.filter(n => n >= Number(target[1])).length
        : 0;
    const failCnt = failures
        ? results.filter(n => n <= Number(failures[1])).length
        : 0;

    return [
        results
            .map(n =>
                target && n >= Number(target[1])
                    ? `\`[${n}]\``
                    : failures && n <= Number(failures[1])
                    ? `\`-${n}-\``
                    : `\`${n}\``
            )
            .join(" "),
        target && !failures
            ? `Successes: **${successCnt}**`
            : target && failures
            ? `Successes: **${
                  successCnt - failCnt
              }** _(${successCnt}-${failCnt})_`
            : null
    ];
};

export default rollDice;
