# 🧶 Loom: A narrative system for {your_game}
Loom is a scripting language that makes writing interactive dialogue fun! It's based on Lisp and powered by Rust, which makes it simple to use and scalable in terms of both performance and narrative complexity.

## Features
### Writeable syntax
Loom is written like instant-message roleplaying and requires very little boilerplate, for example:
```
Loom: This is an example of dialogue!
Player: Seems pretty intuitive.
```

### Inline expressions and string formatting
Write inline expressions `(like this)` for dynamic behavior. String formatting is as simple as using `{curly_braces}` in dialogue!

### Variables
Variables can be read from and written to in dialogue:
```
Loom: What's your name?
(let player_name input)
Loom: Nice to meet you, {player_name}!
```

### Pattern matching
Simple branching dialogue can be easily written with `if` expressions:
```
(if cool (
    Loom: This is radical!
)(
    Loom: Aw, this sucks.
))
```
...or with the `match` expression, which can represent more flexible states:
```
(match temperature (
    ("hot" (
        Loom: It's boiling in here!
    ))
    ("cold" (
        Loom: It's freezing!
    ))
    ("mild" (
        Loom: Ahh, perfect room temp.
    ))
))
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

### Randomization
Branch dialogue randomly in a uniform way with the `random` expression:
```
(random (
    (
        Loom: Option A!
    )
    (
        Loom: Option B?
    )
    (
        Loom: Option C...
    )
))
```
...or, control the likelihood of each option with the `weighted` expression:
```
(weighted (
    (1.0 (
        Loom: This is kinda likely.
    ))
    (3.0 (
        Loom: This is rather likely.
    ))
    (0.1 (
        Loom: This is very unlikely.
    ))
))
```
