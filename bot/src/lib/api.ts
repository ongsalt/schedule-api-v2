import type { Schedule, ScheduleAPIResponse } from "../model/schedule";
import { ApiUrl } from "./url"

export async function getCurrentPeriod(c: string, target: string): Promise<Schedule> {
    try {
        const res = await fetch(ApiUrl.current(c, target))
        const data = await res.json() as ScheduleAPIResponse
        if (data.ok) {
            return data.data
        } else {
            throw new Error("Not found")
        }
    } catch(e) {
        throw e
    }
}

export async function getSpecificPeriod(c: string, day: number, period: number): Promise<Schedule> {
    try {
        const res = await fetch(ApiUrl.specific(c ,day, period))
        return await res.json() as Schedule
    } catch(e) {
        throw e
    }
}
