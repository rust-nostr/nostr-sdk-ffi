import asyncio
from datetime import timedelta
from nostr_sdk import *


async def main():
    # Init logger
    init_logger(LogLevel.INFO)

    client = Client()

    # Add relays and connect
    await client.add_relay("wss://relay.damus.io")
    await client.add_relay("wss://nos.lol")
    await client.connect()

    print("Streaming events from relays...")

    k = Kind(0)
    f = Filter().kind(k).limit(5)

    stream = await client.stream_events(f, timedelta(seconds=10))

    while True:
        event = await stream.next()

        # Check if the stream is terminated
        if event is None:
            break

        print(event.as_json())


    print("Stream terminated.")

if __name__ == '__main__':
    asyncio.run(main())
