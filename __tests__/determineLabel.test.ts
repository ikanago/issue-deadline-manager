import {expect, test} from '@jest/globals';
import {determineLabel} from '../src/label';

test('outdated', () => {
    const deadline = new Date(2021, 10, 1, 9, 0, 0);
    const now = new Date(2021, 10, 2, 9, 0, 0);
    expect(determineLabel(deadline, now)).toEqual('outdated');
});

test('2 day before and less than 48 hours difference', () => {
    const deadline = new Date(2021, 10, 3, 8, 0, 0);
    const now = new Date(2021, 10, 1, 9, 0, 0);
    expect(determineLabel(deadline, now)).toEqual({days: 2});
});

test('2 day before and more than 48 hours difference', () => {
    const deadline = new Date(2021, 10, 3, 10, 0, 0);
    const now = new Date(2021, 10, 1, 9, 0, 0);
    expect(determineLabel(deadline, now)).toEqual({days: 2});
});

test('Just 1 week before 7 days before', () => {
    const deadline = new Date(2021, 10, 8, 9, 0, 0);
    const now = new Date(2021, 10, 1, 9, 0, 0);
    expect(determineLabel(deadline, now)).toEqual({days: 7});
});

test.each([
    [new Date(2021, 10, 13), {weeks: 1}], // Saturday
    [new Date(2021, 10, 14), {weeks: 2}], // Sunday
    [new Date(2021, 10, 20), {weeks: 2}], // Saturday
    [new Date(2021, 10, 21), {weeks: 3}], // Sunday
])('2 weeks before', (deadline, expected) => {
    const now = new Date(2021, 10, 1); // Tuesday
    expect(determineLabel(deadline, now)).toEqual(expected);
});

test.each([
    [new Date(2021, 10, 28), {weeks: 4}], // Sunday
    [new Date(2021, 11, 4), {weeks: 4}], // Saturday
    [new Date(2021, 11, 5), {months: 1}], // Sunday
])('4 weeks before', (deadline, expected) => {
    const now = new Date(2021, 10, 1);
    expect(determineLabel(deadline, now)).toEqual(expected);
});
