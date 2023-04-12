# Paperboy
[Emberry](https://github.com/emberry-org/emberry)'s automatic updater service API.<br>
Designed for use with [Tauri](https://tauri.app).

<br>

## Endpoints
```sh
[  GET  ]  /updates    # returns update information as JSON
[ PATCH ]  /reload     # reloads the update information cache (used for github actions)
```

## Logging
```
~ Paperboy v0.1.0
Waiting for requests at 127.0.0.1:1985
 > Reloaded newspapers, latest version: v1.1.0   '12/04/2023 18:43'
```

<br>

<sub>© 2023 Devensiv & Max, All rights reserved.</sub>