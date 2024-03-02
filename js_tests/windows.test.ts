import { windows_get_all } from "../pkg/chrome_extensions";

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
