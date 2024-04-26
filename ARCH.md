Update



Should i use websokcets vs longpolling:
- I have li


Server works as multiplyer and retranslator
ClientAction -> Server | Saves state -> OtherClientsSync

Client to Server
    ChatMessage 
        - Author, Text
        - Roll
    MapUpdates 
        - MapChanged
        - Token:
            - Moved
            - Added
            - Removed
            - StatusApplied
            - StatusRemoved
        - Object:
            - Moved
            - Added
            - Removed
            - Locked
            - Unlocked
    Music
        - Started
        - Paused
    CharSheet
        - StatUpdated
        - Roll

