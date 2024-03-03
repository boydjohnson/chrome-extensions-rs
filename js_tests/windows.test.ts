import {
  windows_create_callback,
  windows_get,
  windows_get_all,
  windows_get_current,
  windows_get_last_focused,
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
});
