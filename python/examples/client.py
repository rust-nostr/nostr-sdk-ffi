import asyncio
from datetime import timedelta
from nostr_sdk import *


async def main():
    # Init logger
    init_logger(LogLevel.INFO)

    client = Client()

    # Add relays and connect
    relays = [
        "wss://relay.damus.io",
        "wss://nostr.wine",
    ]

    for relay in relays:
        relay = RelayUrl.parse(relay)
        await client.add_relay(relay)

    await client.connect()

    # Generate keys
    keys = Keys.generate()

    # Send an event using the Nostr Signer
    event = EventBuilder.text_note("Test from rust-nostr Python bindings!").sign(keys)
    await client.send_event(event)

    # Mine a POW event and sign it with custom keys
    print("Mining a POW text note...")
    adapter = SingleThreadPow()
    unsigned_event = EventBuilder.text_note("Hello from rust-nostr Python bindings with POW!").build(keys.public_key())
    unsigned_event = await unsigned_event.mine_async(adapter, 20)
    event = unsigned_event.sign(keys)
    output = await client.send_event(event)
    print("Event sent:")
    print(f" hex:    {output.id.to_hex()}")
    print(f" bech32: {output.id.to_bech32()}")
    print(f" Successfully sent to:    {output.success}")
    print(f" Failed to send to: {output.failed}")

    await asyncio.sleep(2.0)

    # Get events from relays
    print("Getting events from relays...")
    f = Filter().authors([keys.public_key(), keys.public_key()])
    events = await client.fetch_events(ReqTarget.auto([f]), timedelta(seconds=10))
    for event in events.to_vec():
        print(event.as_json())


if __name__ == '__main__':
    asyncio.run(main())
