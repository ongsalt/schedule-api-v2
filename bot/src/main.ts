import { ActivityType, Client, GatewayIntentBits, Partials } from "discord.js"
import { config } from "dotenv"

config()

const client = new Client({
    intents: [
        GatewayIntentBits.GuildPresences,
        GatewayIntentBits.GuildMembers,
        GatewayIntentBits.Guilds
    ],
})

client.on('ready', () => {
    if (process.env.device === 'development machine') {
        const presence = client.user!.setActivity('rewrite becuase I can', { type: ActivityType.Watching })
        console.log(`Activity set to ${presence.activities[0].name}`)
    } 
    console.log('[main] Ready')
})

client.on('interactionCreate', async (interaction) => {
    interaction
})

process.on('SIGINT', () => {
    // do something
    client.destroy()
    console.log('destroyed client')
    process.exit();
})

client.login(process.env.CLIENT_TOKEN);
