# Tranquility

Tranquility is a small ActivityPub server written in Rust  
Federation should work with most other implementations (once it actually, you know, federates)  

Progress:

- [ ] Federation
- - [x] Webfinger server
- - [ ] Webfinger client
- - [x] HTTP signature signing
- - [x] HTTP signature verification
- - [ ] Actors
- - - [x] The actor itself
- - - [x] Inbox
- - - [x] Outbox
- - - [x] Following collection
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

- [ ] Client API (I'll probably implement the Mastodon API)
- - [ ] Authorization
- - [ ] Accounts
- - [ ] Get status
- - [ ] Post status
- - - [ ] Media upload
- - [ ] Timelines
- - - [ ] Home
- - - [ ] Local
- - - [ ] Global
