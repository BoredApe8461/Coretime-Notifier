### Coretime Notification API

This is a project related to the RegionX node. It allows parachain teams to subscribe to notifications about coretime-related events that happen on-chain. 

## Interesting Crates to Consider
- [Storage](./services/storage/) => Uses SQLite schema to store data into a filesystem locally. Do not modify the `.db` folder as this is automatically generated and updated with more rows representing the Table's values.
- [API](./services/api/)
- [Tracker](./services/tracker/)

## Contribution Guidelines

