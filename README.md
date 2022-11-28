# 🧶 Loom: A narrative system for {your_game}
Loom is a dynamic dialogue library for games with inline lisp-like expressions!

## Features
### Writeable syntax
Loom is written like instant-message roleplaying and requires very little boilerplate, for example:
```
Loom: This is an example of dialogue!
Player: Seems pretty intuitive.
```

### Inline expressions and string formatting
Write inline expressions `(like this)` for programatic behavior. String formatting is as simple as `Loom: Using {curly_braces} in dialogue!`

### Variables
Variables can be read from and written to in dialogue:
```
Loom: What's your name?
(let player_name input)
Loom: Nice to meet you, {player_name}!
```

### Choices
Adding a player choice is as simple as calling the `choice` function in an expression:
```
Loom: Are you liking this style of writing so far?
(choice
    ("I love it!" (
        Loom: That's great to hear!
    ))
    ("Not really." (
        Loom: Aw, I'm heartbroken.
        (end)
    ))
)
```
