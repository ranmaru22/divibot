# DiviBot

A simple Discord bot written in Rust. Version 0.2 is a complete rewrite
of the original 0.1 which was written in JavaScript. It's not yet
feature-complete and get extended every now and so often.

For now it can do basic dice rolls for pen and paper games.

## Installation & Running

You need to have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

```bash
git clone git@github.com:ranmaru22/divibot.git
cd divibot
cargo build
DISCORD_TOKEN="your token" ./target/debug/divibot
```

## Commands

### Ping

A basic ping command.

```
!ping
> Pong!
```

### Rolling dice

You can trigger a roll with the `!roll` (short: `!r`) command.

```
!r 3d6
> [3, 4, 6]
```

Optional arguments are `-c` for counting successes, and `-e` for exploding
rolls (keep rolling when rolling the target number and sum up the results).

```
!r 3d6 -c6
> [3, 4, 6] - 1 success!

!r 3d6 -c6 -e10
> [3, 4, 11] - 1 success!
```

You can keep only the best N rolls or drop the lowest N rolls of your results
with the `-k` and `-d` flags. In both cases the results will be sorted.

```
!r 5d6 -k2
> [4, 6]

!r 5d6 -d2
> [3, 4, 6]
```

## Credits

- [serenity](https://github.com/serenity-rs/serenity) - Discord API library
- [Tokio](https://tokio.rs) - Asynchronous runtime

## License

DiviBot is licensed under the GPL v3.0 or later. See the LICENSE file for details.
