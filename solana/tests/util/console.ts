import { web3 } from "@coral-xyz/anchor";

export function silenceConsole() {
  const logSpy = jest.spyOn(console, "log").mockImplementation(() => undefined);
  const errorSpy = jest
    .spyOn(console, "error")
    .mockImplementation(() => undefined);

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
  const silencer = isSilent ? silenceConsole() : undefined;

  await expect(fn).rejects.toThrowError(error);

  if (silencer) {
    silencer.close();
  }
}

export function listenSolanaLogs(connection: web3.Connection) {
  const handle = connection.onLogs("all", (logs: web3.Logs) => {
    console.log(logs.logs.join("\n"));
    if (logs.err) {
      console.error(logs.err);
    }
  });

  return {
    close: async () => {
      connection.removeOnLogsListener(handle);
    },
  };
}

export async function dumpAccounts<T>(
  connection: web3.Connection,
  accounts: T
): Promise<T> {
  const lines = [];
  for (const key of Object.keys(accounts)) {
    const account = accounts[key] as web3.PublicKey;
    const info = await connection.getAccountInfoAndContext(account);
    lines.push(
      [key, account.toBase58(), info.value?.owner?.toBase58() || "[none]"].join(
        " "
      )
    );
  }
  console.log(lines.join("\n"));
  return accounts;
}
