import {differenceInCalendarDays, isAfter} from 'date-fns';

type DeadlineLabel = 'outdated' | DaysBefore | WeeksBefore | MonthsBefore;

type DaysBefore = {
    days: number;
};

type WeeksBefore = {
    weeks: number;
};

type MonthsBefore = {
    months: number;
};

const determineLabel = (deadline: Date, now: Date): DeadlineLabel => {
    if (isAfter(now, deadline)) {
        return 'outdated';
    }

    const daysBefore = differenceInCalendarDays(deadline, now);
    if (daysBefore < 7) {
        return {days: daysBefore};
    }

    return {months: 0};
};

export {determineLabel};
