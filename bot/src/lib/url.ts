import { config } from "dotenv"

config()

export function getApiUrl() {
    const url = process.env.API_SERVER
    if (!url || url === undefined) {
        throw new Error("no api url provided")
    }
    return url
}

export const url = getApiUrl()


/**
 *  - /api/schedule/[class]/[target]
 *  - /api/schedule/[class]/[day]/[period]
 */

export const ApiUrl = {
    base: url,
    current: (c: string, target: string) => {
        return `${url}/api/schedule/${c}/${target}`
    },
    specific: (c: string, day: number, period: number) => {
        return `${url}/api/schedule/${c}/${day}/${period}`
    }
}