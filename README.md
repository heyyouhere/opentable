# Open source VTTRPG
This is my attempt to make free as in freedom, self-hosted alternative VTTRPG.

## Usage
I'm planning to make a docker container, so you'll be able to setup Opentable with ease.
Untill then, just build and run the thing.
```console
$ cargo run
```
Site is now available on http://localhost:1583 

## Frames
If you need something extra, you can make your own frame. It is simple <iframe> that integrates into main page. Look at predone to see how it works.
List of Standart Frames
 - [DEV] Music
 - [DEV] Charsheet
 - [DEV] Chat
 - [DEV] PDF-viewer
 - [ ] Dices
 - [ ] Map
 - [ ] MapSelector
 - [ ] Asset Manager
 - [ ] Notes
 - [ ] Token Maker
 - [ ] Settings

## Roll20 Sheets
### -- IN DEVELOPMENT --
You can use Roll20 character sheets out of box. There are planty ready to play sheets at [Roll20 GitHub](https://github.com/Roll20/roll20-character-sheets).
    
## TODO:
 - [DONE] Make a better UI layout. Single page application, served by server.
 - Fix token stamper, allow drop images.
 - Map using konva.js
 - Long Polling updates. Sync between multiple clients.
 - Example server that anyone can play around with.
 - Imbeded token maker 
 - [DONE] Imbeded PDF viewer.
 - DM/Player separation.
 - Playable field.
 - [DEV] Rolls library.
 - Audio streaming.

# Not in scope of project
 - Video and audio chat.
 - Asset store.
 - No high security measures - it's just games, after all.


