# Loom: A dialogue system for {your_game}
Loom is a dynamic dialogue library for games with inline lisp-like expressions!

## Features
### Writeable syntax
Loom is written like a roleplaying script and requires very little boilerplate, for example:
```
Loom: This is an example of dialogue!
You: Seems pretty intuitive.
```

### Variables
Variables can be read from and written to in dialogue and expressions via the `{varName}` accessor syntax.

### Inline expressions
Write inline dynamic expressions like `(log {player_name})` for programatic behavior.

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
