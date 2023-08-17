export type Schedule = {
    subjectName: string,
    subjectCode?: string,
    room?: string
    location: string, // Next / curernt 
    link?: string,
    teachers: string[],
    day: number,
    period: number,
    isInSchoolTime: boolean,
}

export type ScheduleAPIResponse = {
    ok: true,
    data: Schedule
} | {
    ok: false,
    error: string
}