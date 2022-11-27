# Loom: A dialogue engine for {your_game}
Loom is a dialogue system for games with inline lisp-like expressions!

## Features
### Writeable syntax
Loom is written like a roleplaying script and requires very little boilerplate, for example:
```
Me: This is an example of dialogue!
You: Seems pretty intuitive.
```

### Inline expressions
Write inline dynamic expressions like `(log {player_name})` for programatic behavior.

### Variables
Variables can be read from and written to in dialogue and expressions via the `{varName}` accessor syntax.
