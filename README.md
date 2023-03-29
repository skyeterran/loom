# What is Loom?
Loom is a symbolic scripting language designed to make abstract reasoning easier to implement in code. Its goal is to combine the best parts of the Lisp family of languages with modern conveniences in a highly-debuggable and flexible development environment.

# How is Loom different?
## Lispy syntax
In the tradition of Lisps, all loom code is composed of "S-expressions", bracketed lists of symbols which directly describe the AST (abstract syntax tree) of the logic you're writing.
For instance, in a statement-centric language like Python, code to "set the value of variable 'x' to the average of 'a', 'b', and 'c'" you might write:
```
x = (a + b + c) / 3
```
In Loom, you would write:
```
(set x (/ (+ a b c) 3))
```
This might seem like an arbitrary choice, and it's not like modeling computation as a series of instructions is bad - after all, high-level instructions resemble how computers work on a lower level. However, as weird as they look at first glance, using S-expressions has a deep advantage in terms of program design: *you can represent arbitrarily complex information in a single, uniform way.*

## No interchange format
This might sound weird at first but think for a sec: in other languages, how do you persist and share information between different sessions or programs? Well, typically you'd use an interchange format, which is just a markup language for transferring data between different running programs. In Python and JavaScript, for instance, JSON is the go-to choice for its flexibility and simplicity. But in Loom, there's a built-in interchange format: Loom itself.

See, Loom has built in `save` and `load` functions which actually write variables, in Loom code, to files on-disk... and load Loom files from disk into native data in a running program. Any value you write in Loom can be exported, in the same language with which you wrote it, to a file, and back. The simplest example is just saving and loading a list of numbers:
```
(save [0 1 1 2 3 5 8 13 21 34] "fibonacci.loom")
```
...this will create a file whose contents are simply `[0 1 1 2 3 5 8 13 21 34]`, and you can directly load that file into a variable within another Loom program:
```
(load "fibonnaci.loom" fib)
(print fib) ; "[0 1 1 2 3 5 8 13 21 34]"
```
This feature alone not only eliminates a ton of boilerplate code for saving/loading data, but it also means that you don't need to think of persisted data any differently from hand-written or auto-generated data/code you may create during an interactive Loom session. The language is its own data storage format. But the modularity of Loom doesn't stop there!

## Symbolic environments
In programming, we're constantly using symbols to represent variables and, by extension, their values - but most programming languages place severe limits on how you can actually manipulate and interact with symbols themselves; under the hood, of course, most languages use "symbol tables" to perform this association, so if you write the following:
```
a = 69
b = 420
def greet(name):
  print(f"Hello, {name}!")
```
...under the hood, you're producing a symbol table like this:
```
{
  "a" = number (69),
  "b" = number (420),
  "greet" = function ({
    "arguments" = ["name"],
    "logic" = [print (fstring [
      "Hello, ",
      variable ("name"),
      "!"
    ])]
  })
}
```
Now, that's a contrived example, but it's basically what's happening whenever you write a program: any variables you declare or functions you define are simply values bound to specific names (the symbols), and whenever one of those symbols is invoked later on in the program, the value associated with it is recalled and used in its place. This is awesome! It's also almost completely opaque in most languages.

On the other hand, in almost any Lisp, the program's "environment" is basically just the symbol table you're interacting with. If you create a variable with a specific name, it gets recorded in the environment for your later use. If you define a function, it goes into the environment just the same. Basically every Lisp language lets you debug your working environment easily, meaning you can see all of the data and behavior you've created in a single, named data structure.

It's beautiful! But also annoying, because all of those definitions are global for the most part. What if you define 'username' to be "Skye" in one interactive session, then start a new session, define it to mean "Greg" and then try to load up the environment you worked on earlier?
Worst case scenario, you have a conflict in terms of what symbols represent. If you're an experienced coder, you're probably screaming at the screen about how this is a messy practice, and you'd be right for most languages; after all, you need to create specific structures and ways of sharing information between programs that doesn't just rely on global naming conventions, or you'll be in a whole heap of trouble.

But what if the language was designed around this idea of multiply-defined symbols? See, Loom has only a single compound data type: the environment. But unlike similar languages, environments in Loom aren't just a monolithic structure that exists at the global level. Instead, *Loom environments are values themselves.* Consider the following Loom code:
```
(set name "Skye")
(set age 26)
(set species #dog)
(set greeting (fn ()
  (print "Arf!")
))
```
...as you might imagine, this defines several global symbols, including a function definition. Now imagine you save that script to a file named "user.loom" and then, in a fresh Loom session, call `(load "user.loom" user)` to load the value of the script into the variable 'user' - I know what you're thinking: the script has a value? Yep, and if you printed out 'user' you'd see this:
```
{
  (name "Skye")
  (age 26)
  (species #dog)
  (greeting (fn ()
    (print "Arf!")
  ))
}
```
...seem familiar? It's the entire environment / symbol table of the old program, but instead of being global variables, it's a single compound data structure which can be passed around as a value. This bracketed "environment" data structure syntax can also be written directly, like this:
```
(set new_user {
  (name "Greg")
  (age 39)
  (species #accountant)
  (greeting (fn ()
    (print "Good afternoon.")
  ))
})
```
Now, let's have our old user and new user greet each other:
```
(user/greeting) ; "Arf!"
(new_user/greeting) ; "Good afternoon."
```
It's a fun trick, right? To compare it to python, imagine if everything was a dictionary, and if you set any global variables in one script, they were automatically imported as named values in a new dictionary upon loading them into another script. Not only does this eliminate naming conflicts, but it means that *any Loom program's entire set of data and logic can be modularly plugged into any other Loom program, interactively.*

In fact, environments can be nested arbitrarily:
```
(set user {
  (name "Skye")
  (age 26)
  (species #dog)
  (stats {
    (int 12)
    (cha 19)
    (str 8)
  })
})
```
...so in this case, my charisma score is bound to the symbolic path `user/stats/cha` - if that reminds you of a filesystem, that's intentional. All persistent data/logic in a Loom environment lives within a named, nested structure that resembles URL and filesystem paths, which not only ensures uniformity and modularity in terms of your program structure, but it'll also make it possible to debug the entire living memory structure of a running program as a tree; instead of opaque and secret data being passed around, imagine being able to view the whole current state of a program like so:
```
{
  (resolution [800 600])
  (dpi 300)
  (entities [
    (player {
      (position [12, 42])
      (velocity [1, 3])
      (health 100)
    })
    (npc {
      (position [3, 4])
      (velocity [2, 1])
      (health 200)
    })
  ])
}
```
In this case, you could debug the entire state of a game as an explorable and manipulatable tree of information.
