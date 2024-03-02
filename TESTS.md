# Findings from writing unit tests

## windows

| Name           | Tested |
| -------------- | ------ |
| get            | ✅     |
| getAll         | ✅     |
| getCurrent     | ✅     |
| getLastFocused | ✅     |
| create         | ❌     |
| update         |        |
| remove         |        |

### Notes:

create has a callback only, not a promise-based api.
