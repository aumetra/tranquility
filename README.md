# Tranquility

Tranquility is a small ActivityPub server written in Rust  

### **Disclaimer**

Tranquility is far from finished and therefore not ready to be used in any capacity yet  
Backwards incompatible changes might occur  

## Requirements

- **Rust** 1.50+  
- **PostgreSQL** (9.5+ should be fine)

## Custom memory allocators

Tranquility currently supports two custom memory allocators  

Use them by compiling the server with one of the following feature flags:

- `jemalloc`: Use `jemalloc` as the memory allocator
- `mimalloc`: Use `mimalloc` as the memory allocator

These features are mutually exclusive  
If more than one is activated, all selected allocators are compiled in the binary but neither will be actually used  

## Progress

- [ ] Federation
    - [x] Webfinger server
    - [x] Webfinger client
    - [x] HTTP signature signing
    - [x] HTTP signature verification
    - [x] Actors
        - [x] The actor itself
        - [x] Inbox
        - [x] Outbox
        - [x] Following collection
        - [x] Followers collection
    - [x] Activities (incoming)
        - [x] Accept
        - [x] Create
        - [x] Delete
        - [x] Follow
        - [x] Like
        - [x] Reject
        - [x] Undo
        - [x] Update
    - [ ] Activities (outgoing)
        - [x] Accept
        - [ ] Create
        - [ ] Delete
        - [x] Follow
        - [ ] Like
        - [ ] Reject
        - [x] Undo
        - [ ] Update

- [ ] Client API ((mostly) Mastodon API)
    - [x] Authorization
        - [x] App registration
        - [x] OAuth
    - [ ] Accounts
        - [ ] Block
        - [x] Create
        - [ ] Delete
        - [x] Follow
        - [x] Retrieve
        - [ ] Update
    - [ ] Statuses
        - [ ] Delete
        - [ ] Retrieve
        - [ ] Publish
    - [ ] Media upload
    - [ ] Timelines
        - [ ] Home
        - [ ] Local
        - [ ] Global
