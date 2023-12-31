# Features
I can use Anki to create decks, cards, and study them. I can also create a user account and login to save my progress.
I can have sessions where I drill a deck. The results get stored in the database. I can view reports on my progress.
- [o] Card:
    - [o] Create migration file for card:
        - [X] id, name, front, back, created_at, updated_at
    - [ ] Add tags to a card (alternative to decks)
- [X] Create a menu with an event loop
- [O] Deck:
    - [X] Create
    - [X] Update
    - [X] Delete
    - [X] List
    - [X] Add cards to deck
- [ ] User:
    - [ ] choose to log in as guest
    - [ ] CRUD user
    - [ ] use bcrypt to hash passwords
- [ ] Review a deck:
    - [ ] Create a session
    - [ ] Show a card
    - [ ] Grade the card
    - [ ] Show the next card
    - [ ] Save the session
    - [ ] Show a report
- [X] Session:
- [ ] Card history:
    - [ ] User card history
- [ ] User:
    - [ ] CRUD user
    - [ ] auth
- [ ] Reports:
    - [ ] use functions to generate reports
    - [ ] needs to compare given answer with correct answer
    - [ ] a modular solution that can use different algorithms

- [ ] BONUS: Add TUI support for reports
- [ ] BONUS: Create a Tauri app:
    - [ ] with a SvelteKit frontend
- [ ] BONUS: Create a FFI Python API and use it in FastAPI:
    - [ ] Or Typer
- [ ] BONUS: Create a version in Python using Typer:
    - [ ] Extend the Python library code with a version in FastAPI and HTMx
- [ ] BONUS: Create a version in SvelteKit and Firebase
- [ ] BONUS: Create a version in Go using Bubble Tea
- [ ] BONUS: ChatGPT plugin:
    - [ ] I can use GPT-3 to generate cards for me.
    - [ ] I can use GPT-3 to grade my cards for me.
