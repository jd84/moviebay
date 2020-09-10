# moviebay

The Moviebay is a private project in a **very early** stage. The goal is to create a full featured media server such as Plex or Emby and to learn rust.
The backend is written in rust and the frontend is written in JavaScript (VueJS).

> IMPORTANT This project will no support any kind of piracy!

**This project is far away from stable** and is developed in my spare freetime, so updates are infrequent.

## What's included

* ffmpeg is used for live transcoding
* Scan a local folder for movies, atm only one scheme is supported: `Movie Title (2020).{mkv,mp4,avi}`
* sqlite is used to store video meta data such as path, title and so on. Sqlite is not thread-safe so the code exploded a little bit in compexity.
* tmdb (The Movie Database) is used for lookups to get all the cool data such as images original title and description. For the tmdb stuff you need an API-Key.

## Known critical bugs

The process to start the live transcoding with ffmpeg is not good because its a long running block task and I spawn it the wrong way. It's recommandend that long running blocking task should be spawned with `spawn_blocking`.

## Endpoints

* /movies - Get all movies in the database as json
* /movies/:id - Get one movie by id as json
* /stream/:id - Get the live transcoding stream from ffmpeg (file is hardcoded so id doesn't matter atm)

## Requirements

> **Only tested on Linux**

Download ffmpeg binary and put it in the root folder.
Put a video file (best is **h264 mkv**) in the root folder of this project. And change the filename in the code. `src/api/handler.rs:50`.

> If you don't want to scan a local folder or make any lookups to tmdb you've to comment out `src/main.rs:24-54` 

### Start the backend

Simply hit `cargo run`

### Start the frontend

Follow the instructions for the client. https://github.com/jd84/moviebay-client

### Start a stream

You the backend and the frontend running, open your browser and go to http://localhost:8080/play/111. The id ```111``` can be replaced by any number it's not used at the moment.