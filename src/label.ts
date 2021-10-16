import {
    differenceInCalendarDays,
    differenceInCalendarMonths,
    differenceInCalendarWeeks,
    isAfter,
} from 'date-fns';

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

/**
 * Determines label type based on deadline and current time.
 * @param {Date} deadline
 * @param {Date} now
 */
const determineLabel = (deadline: Date, now: Date): DeadlineLabel => {
    if (isAfter(now, deadline)) {
        return 'outdated';
    }

    const daysBefore = differenceInCalendarDays(deadline, now);
    if (daysBefore <= 7) {
        return {days: daysBefore};
    }

    const weeksBefore = differenceInCalendarWeeks(deadline, now);
    if (weeksBefore <= 4) {
        return {weeks: weeksBefore};
    }

    const monthsBefore = differenceInCalendarMonths(deadline, now);
    return {months: monthsBefore};
};

export {determineLabel};

