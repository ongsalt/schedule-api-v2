import { EmbedBuilder } from "discord.js";
import { Schedule } from "../model/schedule";

export function createReply(data: Schedule, padding: string) {
    const embed = new EmbedBuilder()
    embed.setColor('#2f61e0')

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

    if (!data.isInSchoolTime) {
        embed.setTitle("ไม่ได้อยู่ในเวลาเรียน")
        return embed
    }

    embed.setTitle(data.subjectName)

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

export function createErrorMessage(message?: string) {
    const embed = new EmbedBuilder()
    embed.setColor('#2f61e0')
        .setTitle("An error occured")
        .setAuthor({ name: "Bot" })
        .setDescription(message ?? "Service is unreachable. Please ensure that API server is running")
    return embed
}
