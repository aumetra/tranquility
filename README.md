# Tranquility

Tranquility is a small ActivityPub server written in Rust  
Federation should work with most other implementations (once it actually, you know, federates)  

## Important

Do NOT create a database with the name `tranquility_tests` and run the unit tests  
The unit tests will delete all data from the database  

Progress:

- [ ] Federation
- - [ ] Webfinger
- - [x] HTTP signature signing
- - [x] HTTP signature verification
- - [ ] Actors
- - - [x] The actor itself
- - - [x] Inbox
- - - [ ] Outbox
- - - [ ] Following collection
- - - [ ] Followers collection
- - [x] Activities (incoming)
- - - [x] Accept
- - - [x] Create
- - - [x] Delete
- - - [x] Follow
- - - [x] Like
- - - [x] Reject
- - - [x] Undo
- - [ ] Activities (outgoing)
- - - [x] Accept
- - - [ ] Create
- - - [ ] Delete
- - - [ ] Follow
- - - [ ] Like
- - - [ ] Reject
- - - [ ] Undo
