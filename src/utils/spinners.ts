import ora from "ora";

export async function loadWithSpinner<T>(
  promise: Promise<T>,
  remaining: string,
): Promise<T>;
export async function loadWithSpinner<T>(
  promise: Promise<T>,
  presentVerb: string,
  pastVerb?: string,
  remaining?: string,
): Promise<T>;

export async function loadWithSpinner<T>(
  promise: Promise<T>,
  presentVerbOrRemaining: string,
  pastVerb?: string,
  remaining?: string,
): Promise<T> {

    if(!pastVerb) {
        remaining = presentVerbOrRemaining
        presentVerbOrRemaining = "loading";
        pastVerb = "loaded";
    }

  const spinner = ora(`${presentVerbOrRemaining} ${remaining}`).start();
  try {
    const result = await promise;
    spinner.succeed(`${pastVerb} ${remaining}`);
    return result;
  } catch (e) {
    spinner.fail(`Failed to ${pastVerb} ${remaining}`);
    throw e;
  }
}
