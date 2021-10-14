import * as core from '@actions/core';
import {wait} from './wait';

async function run(): Promise<void> {
    try {
        const done = await wait(500);
        core.debug(done);
    } catch (error) {
        if (error instanceof Error) {
            core.setFailed(error.message);
        }
    }
}

run();
