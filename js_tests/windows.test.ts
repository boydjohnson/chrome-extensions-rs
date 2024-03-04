import {
  windows_create,
  windows_create_callback,
  windows_get,
  windows_get_callback,
  windows_get_all,
  windows_get_current,
  windows_get_last_focused,
  windows_get_all_callback,
  windows_get_current_callback,
} from "../pkg/chrome_extensions";

describe("windows.getAll", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it("Should return windows", async () => {
    jest
      .spyOn(chrome.windows, "getAll")
      .mockImplementation((queryOptions: chrome.windows.QueryOptions) =>
        Promise.resolve([
          { id: 123, focused: true, alwaysOnTop: false, incognito: false },
        ]),
      );

    const windows = await windows_get_all(null);

    expect(windows).toEqual([
      { id: 123, focused: true, alwaysOnTop: false, incognito: false },
    ]);
  });
  it("Should return only some windows with queryOptions", async () => {
    jest
      .spyOn(chrome.windows, "getAll")
      .mockImplementation((queryOptions: chrome.windows.QueryOptions) =>
        Promise.resolve(
          queryOptions.windowTypes && queryOptions.windowTypes.includes("popup")
            ? [
                {
                  id: 456,
                  focused: false,
                  alwaysOnTop: true,
                  incognito: true,
                  type: "popup",
                },
              ]
            : [
                {
                  id: 123,
                  focused: true,
                  alwaysOnTop: false,
                  incognito: false,
                  type: "normal",
                },
              ],
        ),
      );

    const queryOptions = { windowTypes: ["popup"] };
    const windows = await windows_get_all(queryOptions);
    expect(windows).toEqual([
      {
        id: 456,
        focused: false,
        alwaysOnTop: true,
        incognito: true,
        type: "popup",
      },
    ]);
  });
});

describe("windows.getAll callback", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it("Should return windows", (done) => {
    jest
      .spyOn(chrome.windows, "getAll")
      .mockImplementation(
        (
          queryOptions?: chrome.windows.QueryOptions,
          callback?: (windows: chrome.windows.Window[]) => void,
        ): Promise<chrome.windows.Window[]> => {
          if (callback) {
            callback([
              { id: 123, focused: true, alwaysOnTop: false, incognito: false },
            ]);
          }

          return Promise.resolve([
            { id: 123, focused: true, alwaysOnTop: false, incognito: false },
          ]);
        },
      );

    const windows = windows_get_all_callback(
      null,
      (windows: chrome.windows.Window[]) => {
        try {
          expect(windows).toEqual([
            { id: 123, focused: true, alwaysOnTop: false, incognito: false },
          ]);
          done();
        } catch (error) {
          done(error);
        }
      },
    );
  });
  it("Should return only some windows with queryOptions", (done) => {
    jest
      .spyOn(chrome.windows, "getAll")
      .mockImplementation(
        (
          queryOptions?: chrome.windows.QueryOptions,
          callback?: (windows: chrome.windows.Window[]) => void,
        ): Promise<chrome.windows.Window[]> => {
          if (callback) {
            queryOptions &&
            queryOptions.windowTypes &&
            queryOptions.windowTypes.includes("popup")
              ? callback([
                  {
                    id: 456,
                    focused: false,
                    alwaysOnTop: true,
                    incognito: true,
                    type: "popup",
                  },
                ])
              : callback([
                  {
                    id: 123,
                    focused: true,
                    alwaysOnTop: false,
                    incognito: false,
                    type: "normal",
                  },
                ]);
          }

          return Promise.resolve(
            queryOptions &&
              queryOptions.windowTypes &&
              queryOptions.windowTypes.includes("popup")
              ? [
                  {
                    id: 456,
                    focused: false,
                    alwaysOnTop: true,
                    incognito: true,
                    type: "popup",
                  },
                ]
              : [
                  {
                    id: 123,
                    focused: true,
                    alwaysOnTop: false,
                    incognito: false,
                    type: "normal",
                  },
                ],
          );
        },
      );

    const queryOptions = { windowTypes: ["popup"] };
    windows_get_all_callback(queryOptions, (windows: chrome.windows.Window) => {
      try {
        expect(windows).toEqual([
          {
            id: 456,
            focused: false,
            alwaysOnTop: true,
            incognito: true,
            type: "popup",
          },
        ]);
        done();
      } catch (error) {
        done(error);
      }
    });
  });
});

describe("windows.GetCurrent", () => {
  it("Should return a window", async () => {
    jest
      .spyOn(chrome.windows, "getCurrent")
      .mockImplementation((queryOptions: chrome.windows.QueryOptions) =>
        Promise.resolve({
          id: 123,
          focused: true,
          alwaysOnTop: false,
          incognito: false,
        }),
      );

    const windows = await windows_get_current(null);

    expect(windows).toEqual({
      id: 123,
      focused: true,
      alwaysOnTop: false,
      incognito: false,
    });
  });

  it("Should return only some windows with queryOptions", async () => {
    jest
      .spyOn(chrome.windows, "getCurrent")
      .mockImplementation((queryOptions: chrome.windows.QueryOptions) =>
        Promise.resolve(
          queryOptions.windowTypes && queryOptions.windowTypes.includes("popup")
            ? {
                id: 456,
                focused: false,
                alwaysOnTop: true,
                incognito: true,
                type: "popup",
              }
            : {
                id: 123,
                focused: true,
                alwaysOnTop: false,
                incognito: false,
                type: "normal",
              },
        ),
      );

    const queryOptions = { windowTypes: ["popup"] };
    const windows = await windows_get_current(queryOptions);
    expect(windows).toEqual({
      id: 456,
      focused: false,
      alwaysOnTop: true,
      incognito: true,
      type: "popup",
    });
  });
});

describe("windows.GetCurrent callback", () => {
  it("Should return a window", (done) => {
    jest
      .spyOn(chrome.windows, "getCurrent")
      .mockImplementation(
        (
          queryOptions?: chrome.windows.QueryOptions,
          callback?: (window: chrome.windows.Window) => void,
        ): Promise<chrome.windows.Window> => {
          if (callback) {
            callback({
              id: 123,
              focused: true,
              alwaysOnTop: false,
              incognito: false,
            });
          }

          return Promise.resolve({
            id: 123,
            focused: true,
            alwaysOnTop: false,
            incognito: false,
          });
        },
      );

    const windows = windows_get_current_callback(
      null,
      (window: chrome.windows.Window) => {
        try {
          expect(window).toEqual({
            id: 123,
            focused: true,
            alwaysOnTop: false,
            incognito: false,
          });
          done();
        } catch (error) {
          done(error);
        }
      },
    );
  });

  it("Should return only some windows with queryOptions", (done) => {
    jest
      .spyOn(chrome.windows, "getCurrent")
      .mockImplementation(
        (
          queryOptions: chrome.windows.QueryOptions,
          callback?: (window: chrome.windows.Window) => void,
        ): Promise<chrome.windows.Window> => {
          if (callback) {
            queryOptions.windowTypes &&
            queryOptions.windowTypes.includes("popup")
              ? callback({
                  id: 456,
                  focused: false,
                  alwaysOnTop: true,
                  incognito: true,
                  type: "popup",
                })
              : callback({
                  id: 123,
                  focused: true,
                  alwaysOnTop: false,
                  incognito: false,
                  type: "normal",
                });
          }

          return Promise.resolve(
            queryOptions.windowTypes &&
              queryOptions.windowTypes.includes("popup")
              ? {
                  id: 456,
                  focused: false,
                  alwaysOnTop: true,
                  incognito: true,
                  type: "popup",
                }
              : {
                  id: 123,
                  focused: true,
                  alwaysOnTop: false,
                  incognito: false,
                  type: "normal",
                },
          );
        },
      );

    const queryOptions = { windowTypes: ["popup"] };
    windows_get_current_callback(
      queryOptions,
      (window: chrome.windows.Window) => {
        try {
          expect(window).toEqual({
            id: 456,
            focused: false,
            alwaysOnTop: true,
            incognito: true,
            type: "popup",
          });
          done();
        } catch (error) {
          done(error);
        }
      },
    );
  });
});

describe("windows.GetLastFocused", () => {
  it("Should return a window", async () => {
    jest
      .spyOn(chrome.windows, "getLastFocused")
      .mockImplementation((queryOptions: chrome.windows.QueryOptions) =>
        Promise.resolve({
          id: 123,
          focused: true,
          alwaysOnTop: false,
          incognito: false,
        }),
      );

    const windows = await windows_get_last_focused(null);

    expect(windows).toEqual({
      id: 123,
      focused: true,
      alwaysOnTop: false,
      incognito: false,
    });
  });

  it("Should return only some windows with queryOptions", async () => {
    jest
      .spyOn(chrome.windows, "getLastFocused")
      .mockImplementation((queryOptions: chrome.windows.QueryOptions) =>
        Promise.resolve(
          queryOptions.windowTypes && queryOptions.windowTypes.includes("popup")
            ? {
                id: 456,
                focused: false,
                alwaysOnTop: true,
                incognito: true,
                type: "popup",
              }
            : {
                id: 123,
                focused: true,
                alwaysOnTop: false,
                incognito: false,
                type: "normal",
              },
        ),
      );

    const queryOptions = { windowTypes: ["popup"] };
    const windows = await windows_get_last_focused(queryOptions);
    expect(windows).toEqual({
      id: 456,
      focused: false,
      alwaysOnTop: true,
      incognito: true,
      type: "popup",
    });
  });
});

describe("windows.get", () => {
  it("Should return a window", async () => {
    jest.spyOn(chrome.windows, "get").mockImplementation((windowId: number) =>
      Promise.resolve({
        id: 123,
        focused: true,
        alwaysOnTop: false,
        incognito: false,
      }),
    );

    const window = await windows_get(123);

    expect(window).toEqual({
      id: 123,
      focused: true,
      alwaysOnTop: false,
      incognito: false,
    });
  });
});

describe("windows.get with callback", () => {
  it("Should pass the window to the callback", (done) => {
    jest
      .spyOn(chrome.windows, "get")
      .mockImplementation(
        (
          windowId: number,
          queryOptions?: chrome.windows.QueryOptions,
          callback?: (window: chrome.windows.Window) => void,
        ): Promise<chrome.windows.Window> => {
          {
            if (callback) {
              callback({
                id: 123,
                focused: true,
                alwaysOnTop: false,
                incognito: false,
              });
            }
            return Promise.resolve({
              id: 123,
              focused: true,
              alwaysOnTop: false,
              incognito: false,
            });
          }
        },
      );

    const window = windows_get_callback(
      123,
      null,
      (window: chrome.windows.Window) => {
        try {
          expect(window).toEqual({
            id: 123,
            focused: true,
            alwaysOnTop: false,
            incognito: false,
          });
          done();
        } catch (error) {
          done(error);
        }
      },
    );
  });
});

describe("windows.create", () => {
  it("Should call the callback with the created window", (done) => {
    jest
      .spyOn(chrome.windows, "create")
      .mockImplementation(
        (
          createInfo: chrome.windows.CreateData,
          callback: (window?: chrome.windows.Window) => any,
        ) =>
          callback({
            id: 123,
            focused: true,
            alwaysOnTop: false,
            incognito: false,
          }),
      );

    windows_create_callback(
      {
        id: 123,
        focused: true,
        alwaysOnTop: false,
        incognito: false,
      },
      (window?: chrome.windows.Window) => {
        try {
          expect(window).toEqual({
            id: 123,
            focused: true,
            alwaysOnTop: false,
            incognito: false,
          });
          done(); // Indicate that the test is complete
        } catch (error) {
          done(error); // Pass error to done if assertion fails
        }
      },
    );
  });

  it("Should return the Window asynchronously", async () => {
    jest
      .spyOn(chrome.windows, "create")
      .mockImplementation(
        (
          createInfo: chrome.windows.CreateData,
          callback?: (window: chrome.windows.Window) => any,
        ): Promise<chrome.windows.Window> => {
          return Promise.resolve({
            id: 123,
            focused: true,
            alwaysOnTop: false,
            incognito: false,
          });
        },
      );

    const window = await windows_create({
      id: 123,
      focused: true,
      alwaysOnTop: false,
      incognito: false,
    });

    expect(window).toEqual({
      id: 123,
      focused: true,
      alwaysOnTop: false,
      incognito: false,
    });
  });
});
