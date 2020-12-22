## Commection Commands

see more: https://redis.io/commands#connection

- [x] AUTH [username] password
- [ ] CLIENT CACHING YES|NO
- [ ] CLIENT GETNAME
- [ ] CLIENT GETREDIR
- [ ] CLIENT ID
- [ ] CLIENT INFO
- [ ] CLIENT KILL [ip:port] [ID client-id] [TYPE normal|master|slave|pubsub] [USER username] [ADDR ip:port] [SKIPME yes/no]
- [ ] CLIENT LIST [TYPE normal|master|replica|pubsub] [ID client-id [client-id ...]]
- [ ] CLIENT PAUSE timeout
- [ ] CLIENT REPLY ON|OFF|SKIP
- [ ] CLIENT SETNAME connection-name
- [ ] CLIENT TRACKING ON|OFF [REDIRECT client-id] [PREFIX prefix [PREFIX prefix ...]] [BCAST] [OPTIN] [OPTOUT] [NOLOOP]
- [ ] CLIENT UNBLOCK client-id [TIMEOUT|ERROR]
- [x] ECHO message
- [ ] HELLO protover [AUTH username password] [SETNAME clientname]
- [x] PING [message]
- [x] QUIT
- [ ] RESET
- [x] SELECT index
