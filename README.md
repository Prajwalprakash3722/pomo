# pomo (Yet Again Another Pomodoro timer in Rust)

## Status

<details>
<summary>Implementation status</summary>

- [x] Parse Arguments
- [ ] Timer
- [ ] send notfication once the timer is completed
- [ ] play a sound once the timer is completed
- [ ] term art
- [ ] let user define their configs

<hr/>

## Arguments

```bash
A pomodoro timer CLI app

Usage: pomo [OPTIONS]

Options:
  -d, --duration <DURATION>
          Sets the duration of a work period in minutes [default: 25]
  -b, --break <BREAK_DURATION>
          Sets the duration of a short break in minutes [default: 5]
  -l, --long-break <LONG_BREAK_DURATION>
          Sets the duration of a long break in minutes after a set number of cycles [default: 15]
  -c, --cycles <CYCLES>
          Sets the number of work periods before a long break [default: 4]
  -h, --help
          Print help
  -V, --version
          Print version
```
