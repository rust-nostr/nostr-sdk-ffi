import asyncio
from nostr_sdk import RelayUrl, RelayStatus, HandleMonitorNotification, init_logger, LogLevel, ClientBuilder, Monitor


class MyMonitorHandler(HandleMonitorNotification):
    async def relay_status_changed(self, relay_url: RelayUrl, status: RelayStatus):
        print(f"Relay {relay_url} status changed to {status}")

async def main():
    init_logger(LogLevel.DEBUG)

    # Create a new monitor
    monitor = Monitor()

    # Create a new client with the monitor
    client = ClientBuilder().monitor(monitor).build()

    # Add some relays
    urls = ["wss://relay.damus.io", "wss://nostr.mom", "wss://nostr.oxtr.dev"]
    for url in urls:
        url = RelayUrl.parse(url)
        await client.add_relay(url)

    # Connect
    await client.connect()

    # If needed, use client.monitor() to get a clone of the monitor instance
    # monitor = client.monitor()

    # Handle monitor notifications
    await monitor.handle_notifications(MyMonitorHandler())

if __name__ == '__main__':
    asyncio.run(main())
