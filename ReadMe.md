# Stonks

[![Github Issues](https://img.shields.io/github/issues/Stonks-Luma-Liberty/wren?logo=github&style=for-the-badge)](https://github.com/Stonks-Luma-Liberty/wren/issues)
[![Github Top Language](https://img.shields.io/github/languages/top/Stonks-Luma-Liberty/wren?logo=rust&style=for-the-badge)](https://www.rust-lang.org/)

Twitter bot mainly focused on streaming tweets of various twitter users into a discord channel

## Table of Contents

- [Features](#features)

- [Environment Variables](#environment-variables)

- [Run Locally](#run-locally)

  - [With Docker](#with-docker)
  - [Without Docker](#without-docker)

## Features

- Stream tweets directly to discord

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`CONSUMER_KEY` - Token required to connect with twitter api

`CONSUMER_SECRET` - Token required to connect with twitter api

`ACCESS_KEY` - Token required to connect with twitter api

`ACCESS_SECRET` - Token required to connect with twitter api

`BEARER_TOKEN` - Token required to connect with twitter api

`DISCORD_WEBHOOK_URL` - Discord Webhook to receive messages from bot

## Run Locally

Clone the project

```bash
  git clone https://github.com/Stonks-Luma-Liberty/wren.git
```

Go to the project directory

```bash
  cd wren
```

```bash
cargo run
```

### With Docker

Use docker-compose to start the bot

```bash
docker-compose up -d --build
```