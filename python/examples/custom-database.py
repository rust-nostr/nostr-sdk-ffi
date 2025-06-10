import asyncio
from nostr_sdk import *
from nostr_sdk import uniffi_set_event_loop
from typing import List


async def main():
    init_logger(LogLevel.INFO)

    uniffi_set_event_loop(asyncio.get_running_loop())

    # Example of custom in-memory database
    class MyDatabase(CustomNostrDatabase):
        def __init__(self):
            self.seen_event_ids = {}
            self.events = {}

        def backend(self) -> str:
            return "my-in-memory-backend"

        async def save_event(self, e: Event) -> SaveEventStatus:
            self.events[e.id()] = e
            return SaveEventStatus.success()

        async def check_id(self, event_id: EventId) -> DatabaseEventStatus:
            if event_id in self.events:
                return DatabaseEventStatus.SAVED
            else:
                return DatabaseEventStatus.NOT_EXISTENT

        async def event_by_id(self, event_id) -> Event | None:
            return self.events.get(event_id, None)

        async def count(self, filter) -> int:
            return 0

        async def query(self, filter) -> Events:
            # Fake algorithm
            return list(self.events.values())[:10]

        async def delete(self, filter):
            return

        async def wipe(self):
            return

    my_db = MyDatabase()
    database = NostrDatabase.custom(my_db)
    client = ClientBuilder().database(database).build()

    await client.add_relay("wss://relay.damus.io")
    await client.connect()

    keys = Keys.parse("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
    print(keys.public_key().to_bech32())

    # Negentropy reconciliation
    f = Filter().author(keys.public_key())
    opts = SyncOptions()
    await client.sync(f, opts)

    # Query events from database
    f = Filter().author(keys.public_key()).limit(10)
    events = await client.database().query(f)
    if events.len() == 0:
        print("Query not found any event")
    else:
        for event in events.to_vec():
            print(event.as_json())


if __name__ == '__main__':
    asyncio.run(main())
