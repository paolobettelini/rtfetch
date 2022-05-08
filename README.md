## Rfetch

![screen](./img.png)

## Configuration

Recompile each time you change the
[config file](./config.toml)

```toml
logo = "arch"
info = [
    "",
    "",
    "<yellow>{host_name}@{host_name}",
    "",
    "<yellow><bold>System</bold>: <white>{system_name}",
    "<yellow><bold>OS</bold>: <white>{os_version}",
    "<yellow><bold>Uptime</bold>: <white>{uptime / 3600} hours, {(60.0 * (uptime as f32 / 3600.0).fract()) as u8} minutes",
    "<yellow><bold>Kernel</bold>: <white>{kernel_version}",
    "<yellow><bold>Cores</bold>: <white>{cores}",
    "<yellow><bold>Memory</bold>: <white>{used_memory / 1000} MB / {total_memory / 1000} MB ({(100.0 * used_memory as f32 / total_memory as f32 + 0.5) as u8} %)",
    "",
    "<red>███<yellow>███<green>███<cyan>███<blue>███<magenta>███<black>███<white>███",
    "<red!>███<yellow!>███<green!>███<cyan!>███<blue!>███<magenta!>███<black!>███<white!>███"
]```