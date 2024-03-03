# Findings from writing unit tests

## windows

| Name             | Tested |
| ---------------- | ------ |
| get              | ✅     |
| get_all          | ✅     |
| get_current      | ✅     |
| get_last_focused | ✅     |
| create           | ❌     |
| update           |        |
| remove           |        |

### Notes:

create has a callback only, not a promise-based api.
