# DataFusion TableProviderFactory for Arrow Flight
A generic `FlightTableFactory` that can integrate any Arrow Flight RPC service
as a `TableProviderFactory`. Relies on a `FlightDriver` trait implementation to
handle the `GetFlightInfo` call and all its prerequisites.

## Flight SQL
This crate includes a `FlightSqlDriver` that has been tested with
[Ballista](https://github.com/apache/datafusion-ballista),
[Dremio](https://github.com/dremio/dremio-oss) and
[ROAPI](https://github.com/roapi/roapi).