RPM -- Rust PacMan
==================

Just a for-fun pacman clone using the bevy engine, for learning purposes.

Eventually, I'd like to aim to have a high degree of accuracy with regard to
the behavior of the original arcade title. But I'm a long way from that at
the moment.

Rough Roadmap
-------------

**Pacman**
 - [x] Basic movement
 - [x] Sprite animation
 - [x] Follow Map
 - [x] Pre-Turn Logic
 - [X] Tunnel support
 - [ ] Dots -- consumption + score
 - [ ] Dots -- speed alterations (frame skip)

**Ghosts**
 - [ ] Blinky frightened
 - [ ] Target-following pathfinding logic
 - [ ] Target-following debug mode
 - [ ] Blinky chase (algorithm)
 - [ ] Blinky scatter (parameter)
 - [ ] Generalize
 - [ ] Other ghosts
 - [ ] Spawn
 - [ ] Eaten + return to house
 - [ ] Speed alterations due to tunnel

**Bonuses**
 - [ ] Cherries etc

**Gameflow**
 - [ ] Lives, etc
 - [ ] Death animation
 - [ ] Per-level parametization
 - [ ] Title screen
 - [ ] Audio
