## Lists Commands

see more: https://redis.io/commands#list

- [ ] BLMOVE source destination LEFT|RIGHT LEFT|RIGHT timeout
- [ ] BLPOP key [key ...] timeout
- [ ] BRPOP key [key ...] timeout
- [x] BRPOPLPUSH source destination timeout
- [x] LINDEX key index
- [x] LINSERT key BEFORE|AFTER pivot element
- [x] LLEN key
- [ ] LMOVE source destination LEFT|RIGHT LEFT|RIGHT
- [x] LPOP key
- [ ] LPOS key element [RANK rank] [COUNT num-matches] [MAXLEN len]
- [x] LPUSH key element [element ...]
- [x] LPUSHX key element [element ...]
- [x] LRANGE key start stop
- [x] LREM key count element
- [x] LSET key index element
- [x] LTRIM key start stop
- [x] RPOP key
- [x] RPOPLPUSH source destination
- [x] RPUSH key element [element ...]
- [x] RPUSHX key element [element ...]
