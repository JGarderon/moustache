# Moustache - the simple and quick text preprocessing application

Notes: 
  - You can find [this page in French](./README_fr.md). __The French documentation is the definitive and most comprehensive version.__
	- This text was translated by OpenAI's [ChatGPT](https://chatgpt.com).

## What is text preprocessing?

Text preprocessing (or pre-processing in proper English) is the act of producing one text from another, similar to what a programming language can do with a module like [Jinja in Python](https://jinja.palletsprojects.com) - which Moustache draws inspiration from for its syntax.

However, it is a limited production action, generally for sanitary purposes (to avoid errors or omissions) or to avoid [tedious repetition](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself). It is not compilation, which is an action that changes [code abstraction](https://en.wikipedia.org/wiki/Compiler). It is also not a language intended to be interpreted to yield results beyond text production. Preprocessing is not [Turing-complete](https://en.wikipedia.org/wiki/Turing_completeness) and neither is Moustache... although, with extensions, you can actually do a lot, just like with a normal language interpreter! But that's not the goal: it's best to avoid it.

__So what exactly can such an application do? What is it intended for?__ The simple answer is: a lot, especially for generating static textual content (HTML, Markdown, XML, etc.), which relies on a few simple concepts:
  - variability: defining variables (which only have one type here: text, like in a shell such as Bash),
  - conditionality: making a block of text or instruction "optional" (conditional),
  - inclusion: introducing a file or a block of text into another,
  - repetition: repeating a block of text as desired.

This may seem simple, and in principle, it is. Yet there is a magical trick to making it all effective and appealing: recursion in text production. In essence, it's the ability for the text to "re-enter" the application that produced it (in a "transparent" manner for the user), to undergo a new treatment according to the same rules... but not with the same content. There are other effects that occur as well, but we will see them in detail later.

In short, Moustache's text preprocessing is all about [macros](https://en.wikipedia.org/wiki/Macro_(computer_science)).

## Installation

Moustache is written in Rust, so you need to have [the compiler installed](https://www.rust-lang.org/learn/get-started) on your machine. If you want a very reduced version of the application (less than a hundred KB), you can add [the UPX utility](https://upx.github.io/) - but it’s not mandatory.

The first step is to clone the repository where the source code is located:

```bash
git clone https://git.nothus.fr/produits/moustache.git
```

Then, execute the `install.sh` file in your terminal:

```bash
chmod +x ./install.sh # make the script executable
./install.sh # installation for the current user
./install.sh +sudo # installation for the whole system (requires administrator privileges)
```

That's it. Moustache is compiled and installed. Let's test that everything is working:

```bash
moustache --version
```

... You should see something like this:

```
julien@debian:~/moustache$ moustache --version
v1.1.0
```

__To get help at any time on your specific version, it’s easy: `moustache --help`__

For those who want to delve into the code: `cargo doc --open --all --all-features --document-private-items`.

## The 3 possible delimiters

As with Jinja, there are three possible delimiters:
  - `{# ... #}` for comments, which will never be displayed or kept in the output (promise),
  - `{{ ... }}` for expressions - the variables that will actually be replaced by text in the output,
  - `{% ... %}` for statements - essentially the "commands" or instructions you indicate.

These delimiters allow you to know, in any source format, what is for Moustache and what is plain text. That is, text that will undergo _no_ transformation (literally none!).

Unlike expressions or comments, statements have several formats depending on their use. Generally, they surround a block of text that can also contain delimiters.

Thus, each time Moustache takes input, if allowed (via the `-r` argument), it can process it according to the found statements and expressions, then reinsert the output content into its input. If there’s nothing more to do or if only one processing is desired, the output is returned.

By juggling with delimiters and this process, you can achieve very complex results.

__Attention: one delimiter cannot contain another. For example: `{# comment {{ my_var }} #}` is invalid while `{# comment #}{{ my_var }}` is valid.__
