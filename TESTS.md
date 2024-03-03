# Findings from writing unit tests

## windows

| Name                      | Tested |
| ------------------------- | ------ |
| get                       | ✅     |
| get_callback              |        |
| get_all                   | ✅     |
| get_all_callback          |        |
| get_current               | ✅     |
| get_current_callback      |        |
| get_last_focused          | ✅     |
| get_last_focused_callback |        |
| create                    | ❌     |
| create_callback           | ✅     |
| update                    |        |
| update_callback           |        |
| remove                    |        |
| remove_callback           |        |

### Notes:

create has a callback only, not a promise-based api.
