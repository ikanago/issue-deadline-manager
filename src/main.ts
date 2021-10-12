import * as core from '@actions/core'
import { wait } from "./wait";
import { Octokit } from "@octokit/core";

async function run(): Promise<void> {
  try {
//     const ms: string = core.getInput('milliseconds')
//     core.debug(`Waiting ${ms} milliseconds ...`) // debug is only output if you set the secret `ACTIONS_RUNNER_DEBUG` to true
    const done = await wait(500);
    console.log(done);

  } catch (error) {
    if (error instanceof Error) {
      core.setFailed(error.message)
    }
  }
}

run()
