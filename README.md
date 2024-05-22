> [!WARNING]
> Not production ready!  Use at your own risk.

# buy_low

A tool for making automated equity purchases via the Charles Schwab Trader API.

> [!IMPORTANT]
> This tool has a number of external dependencies, including having a Schwab trading account, a Schwab developer account, creating a Schwab app, obtaining a refresh token, and so on.
> 

[This](https://www.reddit.com/r/Schwab/comments/1c2ioe1/the_unofficial_guide_to_charles_schwabs_trader/) is an excellent "unofficial guide" to getting started with Schwab's Trader API.  Please note it also makes reference to [a YouTube video](https://www.youtube.com/watch?v=kHbom0KIJwc&t=681s) by Tyler Bowers and a GitHub [repo](https://github.com/tylerebowers/Schwab-API-Python).

# Basic Usage

```
Running in test mode, no orders will be placed.
+--------+----------------+--------------+----------------+
| Equity | Starting Price | Ending Price | Percent Change |
+--------+----------------+--------------+----------------+
| QQQ    | $418.82        | $455.88      | 8.13%          |
+--------+----------------+--------------+----------------+
| SCHA   | $45.95         | $48.72       | 5.69%          |
+--------+----------------+--------------+----------------+
| DIA    | $382.37        | $398.03      | 3.93%          |
+--------+----------------+--------------+----------------+
| SCHD   | $77.24         | $79.50       | 2.84%          |
+--------+----------------+--------------+----------------+
Worst performing equity: SCHD
Maximum amount to spend: $1000
Maximum whole shares of SCHD to purchase: 12
Current cash balance: $3093.37
Test mode, otherwise 12 shares of SCHD would be purchased.
If you're ready, run with --live
Done!
```

Running with `--live` and suitable credentials will place an order.


# .env File

```
SCHWAB_API_KEY=
SCHWAB_APP_SECRET=
SCHWAB_REFRESH_TOKEN=
# Optional
POSTGRES_CONN_STRING=
```

# buy_low.toml

What equities are you interested in purchasing? How much are you willing to invest _per invocation_?

```
# Maximum amount (in dollars) to use for trading
maximum_amount = 1000

# Equities to trade
equities = ['SCHD', 'DIA', 'QQQ', 'SCHA']
```

