import Discord from "discord.js";

const client = new Discord.Client();

client.once("ready", () => {
    console.log("Up and running!");
});

client.on("message", msg => {
    if (msg.content === "!ping") {
        msg.channel.send("Pong!");
    }
});

client.login(process.env.DISCORD_TOKEN)
