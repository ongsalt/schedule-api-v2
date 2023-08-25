import { ActivityType, Client, GatewayIntentBits, Partials } from "discord.js"
import { config } from "dotenv"
import { getCurrentPeriod } from "./lib/api"
import { createReply, createErrorMessage } from "./lib/reply"
import { getApiUrl } from "./lib/url"

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
    console.log(`[main] url: ${getApiUrl()}`)
    console.log('[main] Ready')
})

client.on('interactionCreate', async (interaction) => {
    if (!interaction.isCommand()) {
        return
    }

    switch (interaction.commandName) {
        case 'ping': 
        await interaction.reply("Hello from *somewhere i deploy")
        break
        case 's': {
            await interaction.deferReply();
            const target = interaction.options.data?.at(0)?.value as string ?? "c"
            console.log(target)
            // change later

            if (target === "f") {
                await interaction.editReply("Visit the website for more details.")
                return
            }

            const className = "m6-5"
            try {
                // console.log(interaction.options.data)
                const schedule = await getCurrentPeriod(className, target)
                const reply = createReply(schedule, target)
                await interaction.editReply({
                    embeds: [reply]
                })
            } catch(e) {
                const msg = e?.toString() ?? undefined
                console.log(`[main] ${msg}`)
                console.log(e)
                await interaction.editReply({
                    embeds: [createErrorMessage(msg)]
                })
            }
            break
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
