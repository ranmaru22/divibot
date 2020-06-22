import Discord from "discord.js";

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
                msg.channel.send(`I am currently running version ${process.env.npm_package_version}.`);
                break;
            default: return void 0;
        }
    }
});

bot.login(process.env.DISCORD_TOKEN)
