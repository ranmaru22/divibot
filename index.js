import Discord from "discord.js";
import messageHandler from "./handlers/messageHandler.js";

// Create the bot instance.
const bot = new Discord.Client();

bot.once("ready", () => {
    console.log("Up and running!");
});

bot.on("message", messageHandler);

bot.login(process.env.DISCORD_TOKEN);
