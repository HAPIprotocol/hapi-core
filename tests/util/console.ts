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
