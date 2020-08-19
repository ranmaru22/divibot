# DiviBot

A simple Discord bot written in Rust. Version 0.2 is a complete rewrite
of the original 0.1 which was written in JavaScript. It's not yet
feature-complete and get extended every now and so often.

For now it can do basic dice rolls for pen and paper games.

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

Optional arguments are `-c` for couting successes, and `-e` for exploding
rolls (keep rolling when rolling the target number and sum up the results).

```
!r 3d6 -c6
> [3, 4, 6] - 1 success!

!r 3d6 -e6 -e10
> [3, 4, 11] - 1 success!
```

## Credits

- [serenity](https://github.com/serenity-rs/serenity) - Discord API library
- [Tokio](https://tokio.rs) - Asynchronous runtime

## License

DiviBot is licensed under the GPL v3.0 or later. See LICENSE file for details.
