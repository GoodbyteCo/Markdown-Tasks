# Markdown Tasks

This document is a syntax for todo-lists in Markdown. It proposes a standard that integrates task date, time, and importance. Like [Gruber's original spec](https://daringfireball.net/projects/markdown/), this syntax aims to be readable as-is.
> a Markdown-formatted document should be publishable as-is, as plain text, without looking like it’s been marked up with tags or formatting instructions.

## Structure

A todo-item, like a Markdown list-item, should take up one line. They begin with a hyphen `-` (so that they render as list-items in standard Markdown), followed by the [Checkbox](.#The-Checkbox), followed by a space. Everything else (text, importance, time, and date) is optional, and can occurr in any order—with the exception of the date, which must come last.

```
- <checkbox> [<text>, <importance>, <time>, <date>]
```

The checkbox is most often written as two square-brackets with a space in-between `[ ]` (see [Checkbox syntax](.#The-Checkbox) for all options). Importance is denoted as a series of exclemation points `!`, from none (least urgent) to three (most urgent). Time, which refrences what time the item occurs or needs to occur by (**not** the duration of the item), is denoted with the at-symbol `@`. The date should be formatted as `YYYY-MM-DD`, and wraped in parentheses `(...)`.

```
- [ ] Example todo item !! @8pm (2020-08-11)
```

## The Checkbox

An empty checkbox is written as: `[ ]`. This should produce an `<input type="checkbox">` element.
```
- [ ] Unfinished todo
```
Tasks that have been pulled from previous times can be written as: `->]`, but should still produce an `<input type="checkbox">` element.
```
- ->] Task from last week
```
Tasks that have been pushed to later are written as: `[->`, and create an `<input type="checkbox" disabled>` element.
A completed task has a checked checkbox, and is written as: `[*]`. This produces an `<input type="checkbox" checked>`.
```
- [*] Unfinished todo
```
You can also use `-`, `+`, `x`, `v`, `•`, `@`, `#`, `√`, `~`, or `✓` to mark a box as checked.
```
- [x] This is fine
- [#] This is also fine
- [✓] All good over here
- [@] Everything is done
```

## Text

The text of a todo-item can contain inline Markdown formatting (**bold**, *italic*, [links](https://itty.bitty.site/#a.html/data:text/html;charset=utf-8;bxze64,XQAAAAIBAQAAAAAAAAAeCEUG0O+oKBdZ2an16qclPsVsLFhs2pzAN35/bHmc6Ddvq8IM4lbOVrZC5BW5l5dG325bCokrtxIjlYLBkr75n38PgebbjPKZcMZMotzv8FTXMx24PkQix6dfYYbsceNOzuogCV3U9EM/hoEJecoR5JILpdVhnqsFU0CH7kUqAL/rRZ0M+rIMqPgkg90WZTXTvAzvh2xq8/wt4efihibwnWzBfQLw5Q+S7K+g+Re4o9bUmdX1eM3wwH1WXf/wsQjA), `code`).
```
- [ ] Get this one done *now*
```

## Importance

The urgency of a task can be set a one of four levels: none, low, medium, and high. This is written as zero, one, two, or three exclamaition points in a row, repectivly. The exclamation points can come before or after the text (but not within the text), and a seperating space is optional.
```
- [ ] Very important!!
- [ ] Even more important !!!
- [ ] !A little important
- [*] Not at all important
```

## Timing

Sometimes tasks need to happen at a certain time (like meetings), or before a certain time (like schoolwork). The time a task occurs at can be set by writting an at-symbol `@` followed by the time (no spaces). The time can be written with or without minutes, and with or with out the "AM/PM" (note that if there is no AM or PM, 24-hour time is assumed).

```
- [ ] This task occurs @8pm
- [ ] @6:30am Walk the dog
- [ ] You can do uppercase @10:30PM
- [ ] Or 24-hour @18:00
```

## Date

The date the task is due can be added at the end. It must follow `(YYYY-MM-DD)` format.

```
- [ ] Happy new year (2025-01-01)
- [*] Eat 100 hotdogs (2006-06-25)
```

## Examples

Here are a handful of examples of correct Markdown task items:

```
- [*] Checkout the new Jack White album (2018-03-23)
- [ ] Put out fire!!! @8AM
- ->] Buy new extinguisher
- [ ] Meet Marty for dinner @6:30pm (2020-08-12)
- [-> Do something later
```
