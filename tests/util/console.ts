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
