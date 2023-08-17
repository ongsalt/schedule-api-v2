import { EmbedBuilder } from "discord.js";
import { Schedule } from "../model/schedule";

export function generateReply(data: Schedule, padding: string) {
    const embed = new EmbedBuilder()
    embed.setColor('#2f61e0')
        .setTitle(data.subjectName)

    switch (padding) {
        case 'c':
            embed.setAuthor({ name: 'คาบเรียนปัจจุบัน' });
            break;
        case 'n':
            embed.setAuthor({ name: 'คาบเรียนถัดไป' });
            break;
        case 'p':
            embed.setAuthor({ name: 'คาบเรียนก่อนหน้า' })
            break;
        default:
            embed.setAuthor({ name: 'คาบเรียนถัดไป' })
    }

    let description = ''
    if (data.teachers.length != 0) {
        description += data.teachers.join(', ')
        if (data.room) {
            description += " • "
        }
    }
    if (data.room) {
        description += data.room
    }

    if (data.link) {
        const url = new URL(data.link)
        embed.setURL(url.toString())
        description += `\n\n ${url.toString()}`
    } else {
        description += "\n\n ไม่มีลิงก์"
    }

    embed.setDescription(description)

    if(process.env.device) embed.setColor('#f5da42')

    return embed
}