import {parseIssueBodyOrComment} from '../src/label';
import {expect, test} from '@jest/globals';

test('No command', () => {
    expect(parseIssueBodyOrComment('Blah blah')).toBeNull();
});

test('Lack of /deadline', () => {
    expect(parseIssueBodyOrComment('2021/12/01')).toBeNull();
});

test('/deadline yyyy/MM/dd HH:mm', () => {
    const comment = `Homework
    /deadline 2021/12/01  09:12`;
    expect(parseIssueBodyOrComment(comment)).toEqual(
        new Date(2021, 11, 1, 9, 12)
    );
});

test('/deadline yyyy/MM/dd', () => {
    const comment = `Homework
    /deadline 2021/12/01`;
    expect(parseIssueBodyOrComment(comment)).toEqual(new Date(2021, 11, 1));
});

test('/deadline MM/dd', () => {
    const comment = `Homework
    /deadline 2021/12/01`;
    expect(parseIssueBodyOrComment(comment)).toEqual(new Date(2021, 11, 1));
});

test('/deadline MM/dd', () => {
    const comment = `Homework
    /deadline 2021/12/01`;
    expect(parseIssueBodyOrComment(comment)).toEqual(new Date(2021, 11, 1));
});

test('/deadline M/d', () => {
    const comment = `Homework
    /deadline 2021/3/1`;
    expect(parseIssueBodyOrComment(comment)).toEqual(new Date(2021, 2, 1));
});

test('/deadline yyyy/MM/dd H:m', () => {
    const comment = `Homework
    /deadline 2021/12/01 9:3`;
    expect(parseIssueBodyOrComment(comment)).toEqual(
        new Date(2021, 11, 1, 9, 3)
    );
});

test('Comment does not start with command', () => {
    const comment = `Blah blah /deadline 2021/12/01 09:12`;
    expect(parseIssueBodyOrComment(comment)).toEqual(
        new Date(2021, 11, 1, 9, 12)
    );
});

test('Multiple commands', () => {
    const comment = `Blah blah /deadline 2021/12/01 09:12
    /deadline 2021/11/1
    /deadline 2021/1/1`;
    expect(parseIssueBodyOrComment(comment)).toEqual(new Date(2021, 0, 1));
});
