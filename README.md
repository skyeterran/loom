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
Write inline dynamic expressions like `(print var_name)` for programatic behavior. String formatting is as simple as using the `Loom: I'm {loom_age} years old!` syntax.

### Variables
Variables can be read from and written to in dialogue:
```
Loom: What's your name?
(let player_name input)
Loom: Nice to meet you, {player_name}!
```

### Choices
Adding a player choice is as simple as calling the `->` function in an expression:
```
Loom: Do you think my arrow syntax is cool?
(-> "Yes." (
    Loom: Glad to hear it!
))
(-> "Not really." (
    Loom: Aw, I'm heartbroken.
    (end)
))
```
