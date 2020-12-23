## Keys Commands

see more: https://redis.io/commands#generic

- [ ] COPY source destination [DB destination-db] [REPLACE]
- [x] DEL DEL key [key ...]
- [ ] DUMP key
- [ ] EXISTS key [key ...]
- [x] EXPIRE key seconds
- [ ] EXPIREAT key timestamp
- [x] KEYS pattern
- [ ] MIGRATE host port key|"" destination-db timeout [COPY] [REPLACE] [AUTH password] [AUTH2 username password] [KEYS key [key ...]]
- [ ] MOVE key db
- [ ] OBJECT subcommand [arguments [arguments ...]]
- [x] PERSIST key
- [x] PEXPIRE key milliseconds
- [ ] PEXPIREAT key milliseconds-timestamp
- [x] PTTL key
- [x] RANDOMKEY
- [x] RENAME key newkey
- [x] RENAMENX key newkey
- [ ] RESTORE key ttl serialized-value [REPLACE] [ABSTTL] [IDLETIME seconds] [FREQ frequency]
- [ ] SCAN cursor [MATCH pattern] [COUNT count] [TYPE type]
- [ ] SORT key [BY pattern] [LIMIT offset count] [GET pattern [GET pattern ...]] [ASC|DESC] [ALPHA] [STORE destination]
- [ ] TOUCH key [key ...]


