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
  error?: string | jest.Constructable | RegExp | Error
) {
  const silencer = silenceConsole();

  await expect(fn).rejects.toThrowError(error);

  silencer.close();
}
