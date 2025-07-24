# ETL Pipeline for Pump.fun Account Data
This repo uses an Arroyo WebSocket source to stream data. It parses the incoming data using a user-defined function (UDF) that deserializes it with Borsh. The processed data is then sent to Kafka (in KRaft mode) in Debezium format.

<img width="3053" height="841" alt="image" src="https://github.com/user-attachments/assets/1e39764a-113a-479e-b6bc-4caa872d04fc" />

Kafka Connect runs in a distributed setup, with connectors configured to sink data into both PostgreSQL and TimescaleDB.


Link for connector class
- curl -OL https://repo1.maven.org/maven2/io/debezium/debezium-connector-jdbc/3.2.0.Final/debezium-connector-jdbc-3.2.0.Final-plugin.tar.gz
- curl -OL https://repo1.maven.org/maven2/io/debezium/debezium-connector-postgres/2.3.2.Final/debezium-connector-postgres-2.3.2.Final-plugin.tar.gz


# Kakfa Sink Setup
- Kakfa running is kraft mode.
- kakfa connector deploye in distributed setup with config file
```
# Basic Connect configuration
bootstrap.servers=localhost:9092
group.id=connect-cluster
key.converter=org.apache.kafka.connect.json.JsonConverter
value.converter=org.apache.kafka.connect.json.JsonConverter
key.converter.schemas.enable=false
value.converter.schemas.enable=false

# Offset storage
offset.storage.topic=connect-offsets
offset.storage.replication.factor=1
offset.storage.partitions=25

# Config storage
config.storage.topic=connect-configs
config.storage.replication.factor=1

# Status storage
status.storage.topic=connect-status
status.storage.replication.factor=1
status.storage.partitions=5

# Plugin path
plugin.path=./config/plugins

# REST API
rest.host.name=localhost
rest.port=8083
```

make sure you plugins are in correct path
