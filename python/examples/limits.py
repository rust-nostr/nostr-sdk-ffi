from nostr_sdk import ClientBuilder, RelayLimits

# Custom relay limits
limits = RelayLimits().event_max_size(128000)

# OR, disable all limits
l = RelayLimits.disable()

client = ClientBuilder().relay_limits(l).build()

# ...
