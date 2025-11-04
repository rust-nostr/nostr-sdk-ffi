import asyncio
from nostr_sdk import *


async def main():
    builder = RelayBuilder().port(7676)

    relay = LocalRelay(builder)

    await relay.run()

    print(f"Relay url: {await relay.url()}")

    while True:
        await asyncio.sleep(60)


if __name__ == '__main__':
    asyncio.run(main())
