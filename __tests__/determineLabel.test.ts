import {expect, test} from '@jest/globals';
import {determineLabel} from '../src/label';

test('outdated', () => {
    const deadline = new Date(2021, 10, 12, 9, 0, 0);
    const now = new Date(2021, 10, 13, 9, 0, 0);
    expect(determineLabel(deadline, now)).toEqual('outdated');
});

test('2 day before and less than 48 hours difference', () => {
    const deadline = new Date(2021, 10, 15, 8, 0, 0);
    const now = new Date(2021, 10, 13, 9, 0, 0);
    expect(determineLabel(deadline, now)).toEqual({days: 2});
});

test('2 day before and more than 48 hours difference', () => {
    const deadline = new Date(2021, 10, 15, 10, 0, 0);
    const now = new Date(2021, 10, 13, 9, 0, 0);
    expect(determineLabel(deadline, now)).toEqual({days: 2});
});
