import asyncio
from nostr_sdk import PublicKey, ClientBuilder, Filter, Kind, init_logger, LogLevel, AdmitPolicy, AdmitStatus, Event, \
    RelayUrl, uniffi_set_event_loop
from datetime import timedelta

class Filtering(AdmitPolicy):
    def __init__(self):
        self.muted_public_keys = set()

    def mute(self, pk: PublicKey):
        self.muted_public_keys.add(pk)

    async def admit_connection(self, relay_url: RelayUrl) -> AdmitStatus:
        return AdmitStatus.success()

    async def admit_event(self, relay_url: RelayUrl, subscription_id: str, event: Event) -> AdmitStatus:
        if event.author() in self.muted_public_keys:
            return AdmitStatus.rejected()
        else:
            return AdmitStatus.success()

async def main():
    uniffi_set_event_loop(asyncio.get_running_loop())

    # Init logger
    init_logger(LogLevel.INFO)

    muted_public_key = PublicKey.parse("npub1l2vyh47mk2p0qlsku7hg0vn29faehy9hy34ygaclpn66ukqp3afqutajft")
    other_public_key = PublicKey.parse("npub1xtscya34g58tk0z605fvr788k263gsu6cy9x0mhnm87echrgufzsevkk5s")

    filtering = Filtering()
    filtering.mute(muted_public_key)

    # Init client
    client = ClientBuilder().admit_policy(filtering).build()

    url = RelayUrl.parse("wss://relay.damus.io")
    await client.add_relay(url)

    await client.connect()

    # Get events
    f = Filter().authors([muted_public_key, other_public_key]).kind(Kind(0))
    events = await client.fetch_events(f, timedelta(seconds=10))
    print(f"Received {events.len()} events")


if __name__ == '__main__':
    asyncio.run(main())
