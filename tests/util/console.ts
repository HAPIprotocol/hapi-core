import { web3 } from "@project-serum/anchor";

export function silenceConsole() {
  const logSpy = jest.spyOn(console, "log").mockImplementation(() => {});
  const errorSpy = jest.spyOn(console, "error").mockImplementation(() => {});

  return {
    close: () => {
      logSpy.mockRestore();
      errorSpy.mockRestore();
    },
  };
}

export async function expectThrowError(
  fn: () => Promise<unknown>,
  error?: string | jest.Constructable | RegExp | Error,
  isSilent = true
) {
  let silencer = isSilent ? silenceConsole() : undefined;

  await expect(fn).rejects.toThrowError(error);

  if (silencer) {
    silencer.close();
  }
}

export function listenSolanaLogs(connection: web3.Connection) {
  const handle = connection.onLogs(
    "all",
    (logs: web3.Logs, ctx: web3.Context) => {
      console.log(logs.logs.join("\n"));
      if (logs.err) {
        console.error(logs.err);
      }
    }
  );

  return {
    close: async () => {
      connection.removeOnLogsListener(handle);
    },
  };
}
