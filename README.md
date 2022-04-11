# nipplebot.rs

[![Build](https://github.com/f4bio/nipplebot.rs/actions/workflows/build.yml/badge.svg)](https://github.com/f4bio/nipplebot.rs/actions/workflows/build.yml)
[![Containerize](https://github.com/f4bio/nipplebot.rs/actions/workflows/containerize.yml/badge.svg)](https://github.com/f4bio/nipplebot.rs/actions/workflows/containerize.yml)
[![codecov](https://codecov.io/gh/f4bio/nipplebot.rs/branch/main/graph/badge.svg?token=RAGGFAZE0Y)](https://codecov.io/gh/f4bio/nipplebot.rs)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)

## table of contents

<!-- START doctoc -->

<!-- END doctoc -->

## about

- incredibly early development -> don't use for anything...

## requirements

// TODO: more details needed

- [youtube-dl](https://github.com/ytdl-org/youtube-dl)
  - `apt install youtube-dl`
- [ffmpeg](https://github.com/FFmpeg/FFmpeg)
  - Option 1: `apt install libavutil-dev libavdevice-dev libavfilter-dev libavformat-dev`
  - Option 2: `apt install ffmpeg`
- [opus](https://github.com/xiph/opus)
  - `apt install libopus-dev`
- [zeromq](https://zeromq.org)
  - `apt install libzmq3-dev`

## helpful links

- ...

## environment

### docker

1. [nats](http://nats.io):
   ```bash
   docker run \
      -it \
      --restart=unless-stopped \
      --name=nats \
      -p 4222:4222 \
      -p 8222:8222 \
      -d nats:latest
   ```

### discord

#### auth

get `DISCORD_TOKEN` from:

`https://discord.com/developers/applications/<application_id>/bot`

#### invite

get invite link from:
`https://discord.com/developers/applications/<application_id>/oauth2`

e.g.:
`https://discord.com/api/oauth2/authorize?client_id=<client_id>&permissions=0&scope=bot%20applications.commands`

#### message's author's guild's channels (the fuck? xD)

response:

```
[
  GuildChannel {
    id: ChannelId(<channel_id>),
    bitrate: None,
    category_id: None,
    guild_id: GuildId(<guild_id>),
    kind: Category,
    last_message_id: None,
    last_pin_timestamp: None,
    name: "Text Channels",
    permission_overwrites: [],
    position: 0,
    topic: None,
    user_limit: None,
    nsfw: false,
    slow_mode_rate: None,
    rtc_region: None,
    video_quality_mode: None,
    message_count: None,
    member_count: None,
    thread_metadata: None,
    member: None,
    default_auto_archive_duration: None
  },
  GuildChannel {
    id: ChannelId(<channel_id>),
    bitrate: None,
    category_id: None,
    guild_id: GuildId(<guild_id>),
    kind: Category,
    last_message_id: None,
    last_pin_timestamp: None,
    name: "Voice Channels",
    permission_overwrites: [],
    position: 0,
    topic: None,
    user_limit: None,
    nsfw: false,
    slow_mode_rate: None,
    rtc_region: None,
    video_quality_mode: None,
    message_count: None,
    member_count: None,
    thread_metadata: None,
    member: None,
    default_auto_archive_duration: None
  },
  GuildChannel {
    id: ChannelId(<channel_id>),
    bitrate: None,
    category_id: Some(ChannelId(<channel_id>)),
    guild_id: GuildId(<guild_id>),
    kind: Text,
    last_message_id: Some(MessageId(<message_id>)),
    last_pin_timestamp: None,
    name: "general",
    permission_overwrites: [],
    position: 0,
    topic: None,
    user_limit: None,
    nsfw: false,
    slow_mode_rate: Some(0),
    rtc_region: None,
    video_quality_mode: None,
    message_count: None,
    member_count: None,
    thread_metadata: None,
    member: None,
    default_auto_archive_duration: None
  },
  GuildChannel {
    id: ChannelId(<channel_id>),
    bitrate: Some(64000),
    category_id: Some(ChannelId(<channel_id>)),
    guild_id: GuildId(<guild_id>),
    kind: Voice,
    last_message_id: None,
    last_pin_timestamp: None,
    name: "General",
    permission_overwrites: [],
    position: 0,
    topic: None,
    user_limit: Some(0),
    nsfw: false,
    slow_mode_rate: None,
    rtc_region: None,
    video_quality_mode: None,
    message_count: None,
    member_count: None,
    thread_metadata: None,
    member: None,
    default_auto_archive_duration: None
  },
  GuildChannel {
    id: ChannelId(<channel_id>),
    bitrate: None,
    category_id: Some(ChannelId(<channel_id>)),
    guild_id: GuildId(<guild_id>),
    kind: Text,
    last_message_id: Some(MessageId(<message_id>)),
    last_pin_timestamp: None,
    name: "events",
    permission_overwrites: [
      PermissionOverwrite {
        allow: ADD_REACTIONS,
        deny: SEND_MESSAGES,
        kind: Role(RoleId(<role_id>))
      }
    ],
    position: 2,
    topic: None,
    user_limit: None,
    nsfw: false,
    slow_mode_rate: Some(0),
    rtc_region: None,
    video_quality_mode: None,
    message_count: None,
    member_count: None,
    thread_metadata: None,
    member: None,
    default_auto_archive_duration: None
  }
 ]
```
