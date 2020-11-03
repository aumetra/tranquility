# Tranquility

Tranquility is a small ActivityPub server written in Rust  
Federation should work with most other implementations (once it actually, you know, federates)  

Progress:

- [ ] Federation
- - [x] Webfinger server
- - [x] Webfinger client
- - [x] HTTP signature signing
- - [x] HTTP signature verification
- - [x] Actors
- - - [x] The actor itself
- - - [x] Inbox
- - - [x] Outbox
- - - [x] Following collection
- - - [x] Followers collection
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

- [ ] Client API (Mastodon API)
- - [ ] Authorization
- - - [ ] App registration
- - - [ ] OAuth
- - [ ] Accounts
- - - [ ] Block
- - - [x] Create
- - - [ ] Delete
- - - [ ] Follow
- - - [ ] Retrieve
- - - [ ] Update
- - [ ] Statuses
- - - [ ] Delete
- - - [ ] Retrieve
- - - [ ] Publish
- - [ ] Media upload
- - [ ] Timelines
- - - [ ] Home
- - - [ ] Local
- - - [ ] Global
