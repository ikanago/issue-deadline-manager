import {
    differenceInCalendarDays,
    differenceInCalendarMonths,
    differenceInCalendarWeeks,
    isAfter,
} from 'date-fns';
import {zonedTimeToUtc} from 'date-fns-tz';

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
 * @param {string} timeZone Time zone of the `deadline`. An offset from UTC or IANA time zone.
 * @returns {DeadlineLabel} The kind of label to apply.
 */
const determineLabel = (
    deadline: Date,
    now: Date,
    timeZone: string = 'UTC'
): DeadlineLabel => {
    const adjustedDeadline = adjustDeadlineTimezone(deadline, timeZone);

    if (isAfter(now, adjustedDeadline)) {
        return 'outdated';
    }

    const daysBefore = differenceInCalendarDays(adjustedDeadline, now);
    if (daysBefore <= 7) {
        return {days: daysBefore};
    }

    const weeksBefore = differenceInCalendarWeeks(adjustedDeadline, now);
    if (weeksBefore <= 4) {
        return {weeks: weeksBefore};
    }

    const monthsBefore = differenceInCalendarMonths(adjustedDeadline, now);
    return {months: monthsBefore};
};

const adjustDeadlineTimezone = (deadline: Date, timeZone: string): Date => {
    return zonedTimeToUtc(deadline, timeZone);
};

export {determineLabel};
