import Discord from "discord.js";

import rollDice from "./modules/rollDice.js";

// Create the bot instance.
const bot = new Discord.Client();

// Set command prefix for the bot.
const PREFIX = "!";

bot.once("ready", () => {
    console.log("Up and running!");
});

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
