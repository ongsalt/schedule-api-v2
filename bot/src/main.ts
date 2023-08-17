import { ActivityType, Client, GatewayIntentBits, Partials } from "discord.js"
import { config } from "dotenv"
import { getCurrentPeriod } from "./lib/api"
import { generateReply } from "./lib/reply"

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
    if (!interaction.isCommand()) {
        return
    }

    switch (interaction.commandName) {
        case 'ping': await interaction.reply("Hello from *somewhere i deploy")
        case 's': {
            console.log(interaction.options.data[0].value)
            // const target = "c"
            const target = interaction.options.data[0].value! as string
            const className = "m6-5"
            // console.log(interaction.options.data)
            const schedule = await getCurrentPeriod(className, target)
            const reply = generateReply(schedule, target)
            interaction.reply({
                embeds: [reply]
            })

        }
    }
})

process.on('SIGINT', () => {
    // do something
    client.destroy()
    console.log('destroyed client')
    process.exit()
})

client.login(process.env.CLIENT_TOKEN)
