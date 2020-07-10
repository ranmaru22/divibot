import Discord from "discord.js";

// Create the bot instance.
const bot = new Discord.Client();

// Set command prefix for the bot.
const PREFIX = "!";

bot.once("ready", () => {
    console.log("Up and running!");
});
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
                  (acc, v) => (v === sides ? acc + v : acc),
                  0
              )}`
            : null,
        showStats
            ? `Botches: ${results.reduce(
                  (acc, v) => (v === 1 ? acc + v : acc),
                  0
              )}`
            : null
    ];
};
bot.on("message", msg => {
    if (msg.content.substring(0, PREFIX.length) !== PREFIX) {
        return void 0;
    } else {
        const argv = msg.content.substring(PREFIX.length).split(" ");
        switch (argv[0]) {
            case "ping":
                msg.channel.send("Pong!");
                break;
            case "version":
                msg.channel.send(
                    `I am currently running version ${process.env.npm_package_version}.`
                );
                break;
            case "roll":
            case "r":
                msg.channel.send(
                    rollDice(argv[1], argv[2] === "--stats" || argv[2] === "-s")
                );
                break;
            default:
                return void 0;
        }
    }
});
bot.login(process.env.DISCORD_TOKEN);
