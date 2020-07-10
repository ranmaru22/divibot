import rollDice from "../modules/rollDice.js";

// Set command prefix for the bot.
const PREFIX = "!";

const messageHandler = msg => {
    if (msg.content.substring(0, PREFIX.length) !== PREFIX) {
        return void 0;
    } else {
        const argv = msg.content.substring(PREFIX.length).split(" ");
        switch (argv[0]) {
            case "ping":
                msg.channel.send("Pong!");
                break;
            case "version":
            case "v":
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
};

export default messageHandler;
