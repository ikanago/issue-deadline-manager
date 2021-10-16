import {parse} from 'date-fns';

/**
 * Parse an issue body or comment which may contain command to specify deadline.
 * The command format is `/deadline ${DATE} ${TIME}`.
 * DATE format is 'yyyy-MM-dd' or 'MM-dd'.
 * TIME is optional. The format is `HH-mm`.
 * If there are multiple commands in the content, adopt the last one.
 * @param {Date} content An issue body or comment
 */
const parseIssueBodyOrComment = (content: string): Date | null => {
    const parsedDates = content
        .split('\n')
        .map(line => parseCommand(line))
        .map((date: Date | null) => {
            if (date instanceof Date) {
                if (Number.isNaN(date.getTime())) {
                    return null;
                }
            }
            return date;
        })
        .filter((date: Date | null): date is Date => Boolean(date));

    if (parsedDates.length === 0) {
        return null;
    }
    return parsedDates[parsedDates.length - 1];
};

const parseCommand = (line: string): Date | null => {
    const tokens = line.split(' ').filter(token => token !== '');
    const commandTokenIndex = tokens.indexOf('/deadline');
    if (commandTokenIndex === -1) {
        return null;
    }

    const [date, time] = tokens.slice(commandTokenIndex + 1);
    if (date === undefined) {
        return null;
    }

    let parsedDate;
    if (time != null) {
        parsedDate = parse(`${date} ${time}`, 'yyyy/MM/dd HH:mm', new Date());
    } else {
        parsedDate = parse(date, 'yyyy/MM/dd', new Date());
    }
    return parsedDate;
};

export {parseIssueBodyOrComment};
