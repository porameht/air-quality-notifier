# Air Quality Notifier

Automated air quality monitoring with Telegram notifications.

## Quick Start

```bash
# Setup
cp .env.example .env
# Edit .env with your API keys

# Run
cargo run
```

## Configuration

Get your [IQAir API key](https://www.iqair.com/air-pollution-data-api) and [Telegram Bot token](https://t.me/botfather).

```env
IQAIR_API_KEY=your_api_key
TELEGRAM_TOKEN=your_bot_token
TELEGRAM_CHANNEL=your_channel_id
CITIES=Ban Suan,Chon Buri
STATE=Chon Buri
COUNTRY=Thailand
CRON_SCHEDULE=0 0 */3 * * *  # Every 3 hours
```

## Air Quality Levels

| PM2.5 (µg/m³) | Level | Icon |
|---------------|-------|------|
| 0-25 | Good | 🟢 |
| 26-37 | Moderate | 🟡 |
| 38-50 | Unhealthy for Sensitive | 🟠 |
| 51-90 | Unhealthy | 🔴 |
| 90+ | Very Unhealthy | 🟣 |

## License

MIT
