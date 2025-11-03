# Document

A document is the serialsied form of an item. In other words, it is the concrete implementation of the item; the item abstracts over arbitrary document types.

This aims to enable support for arbitrary plaintext formats, such as Markdown, Org, ReStructured Text, AsciiDoc, etc.

Ideally, this is externalised some plugin system, or an external binary, each document handler should be able to provide reflection capabilities, such as querying arbitrary syntax patterns (_e.g._ find a link whose label matches a given pattern that is inside a heading of a given level whose label matches another pattern).

# Description

A description is a string, which may optionally reference tasks, projects, or items (by identifier).

An issue with this is that these identifier markers are not native Markdown elements: it might be necessary to deviate somewhat. A possible solution to this issue is to treat each document as a syntax tree, compressed into records. This effectively flattens the two-tier structure of the document: the metadata and the contents are no longer separate. For example,

```md
---
title: "A document"
tags:
  - a
  - b
  - c
---

# First heading

This is a heading with contents

## A subheading

This is subheading underneath the first heading. It also has a [link](another-document.md).
```


gets parsed into

```yaml
title: "A document"
tags:
  - a
  - b
  - c
contents:
  - label: "First heading"
    type: "heading"
    level: 1
    children:
      - label: "This is a heading with contents"
        type: "paragraph"
      - label: "A subheading"
        type: "heading"
        level: 2
        children:
          - label: "This is subheading underneath the first heading. It also has a [link](another-document.md)."
            type: "paragraph"
            children:
              - label: "link"
                type: "link"
                destination: "another-document.md"
```

The metadata can then be extracted from the `contents` section, keeping track of information such as if this doucment links to another. It is important that this metadata is trivially and deterministically derivable from `contents`.

# Item

An item is an overarching abstraction of the possible objects in this software. It consists of

- an identifier,
- a description,
- a set of tags,
- a timestamp indicating its creation time, and
- a timestamp indicating its most recent modification time.

# Task

A task is an _item_ consisting of

- a state (ready, in progress, done, blocked, waiting),
- a due date,
- a "done" timestamp,
- an optional recurrence interval,
- a list of dependencies,
- a _log_ of time intervals in which it was worked on (with an optional associated description or note for each interval),
- a _log_ of the state changes (excluding undos),
- a weight (this would be equivalent to _priorities_ in traditional task management utilities),
- an effort estimate, and
- optionally, an associated project.

*Note.* Look into implementing a way to calculate a task's late penalty as a function of its parameters

## Dependencies

Given a task X, a dependency of X is a task Y such that X cannot be completed unless the state of X is "done".

## State

The state of a task is an encapsulation of the possible operations on and behaviours of said task. Namely,

- a *ready* task is one which is not being actively worked on, but _can_ be,
- a *waiting* task is one which is not being actively worked on, and _cannot_ be until some external condition is fulfilled; consequently, the change of state from *waiting* to *ready* is manually managed by the user,
- a *blocked* task is one which is not being actively worked on, and _cannot_ be until all of its _dependencies_ are completed; consequently, the change of state from *blocked* to *ready* cannot be modified by the user, and is instead managed by the software,
- an *in progress* task is one which is being actively worked on, and can transition into any of the other states, and
- a *done* task is one whcih is not being actively worked on, and requires no further action.

There can only be one task that is _in progress_ globally. Switching a task to *in progress* will set any other *in progress* tasks to *ready*.

# Log

**Note.** Look into separating this into a layer underneath `moirai`'s state in order to construct a CRDT.

A log is an _item_ consisting of

- a start time, and
- an associated task.

## Progress log

A progress log is a _log_ which records when a task is worked on, _i.e._ when and how long a task's state is set as *in progress*. It consists of

- an end time.

## State change log

A state change log is a _log_ which records when a task's state is changed. It consists of

- the origin state, and
- the destination state.


# Priority scores

The priority of a task should reflect

- the effort-to-impact ratio of the task,
- the cost of switching to the task, and
- the readiness of the task.

On a more abstract level, the score reflects the scheduling system's _constraints_ , namely

- switching cost (formally _sequence dependent startup cost_, though this can be influenced by a job's _project_, formally _family_, as well; it doesn't take as much effort to swap between one task to another of the same project),
- and readiness, _i.e._ how likely it is for a task to stop being *blocked* any time soon, formally the _precedence constraints_, and

its _objectives_, which vary from metrics such as

- *makespan*, which is the completion time of the last job to leave the system,
- *maximum lateness*, which is the worst violation of the due dates,
- *total weighted completion time*, which is the sum of the product of every task's weight with its completion time,
- *total weighted tardiness*, which is the sum of the product of every task's weight with its tardiness, or
- *weighted number of tardy jobs*, which is the sum of the weights of tardy jobs

I am more personally biased to _lateness_ as a metric over _tardiness_, as is accounts for "earliness" as well. Although in a more pragmatic sense, it does not matter if a task is completed before or on precisely on the deadline, and one could argue that measuring tardiness would promote more rest to the user, it is also the case that, unlike in traditional scheduling problems, the user is not a machine, and has autonomy. The priority score acts as mere _suggestions_, and the user can choose to rest instead. On the contrary, [Parkinson's law](https://en.wikipedia.org/wiki/Parkinson%27s_law) suggests that it would be wiser for such a prioritisation system to avoid "giving time" to the user.

*Note.* Can we model breakdowns as well, _i.e._ the unavailability of the machine (user)? I'm guessing we can model scheduled _rest_, but not mandatory unavailabilities such as illness.

## Effort-to-impact ratio

This is what is traditionally considered to be the objective of a scheduling problem.

## Readiness

With tasks which are blocked by its dependencies, it is worthwhile to discriminate between its 'readiness' - that is, how likely that task is to be ready soon - as to improve spatial locality in the user's mind. As tasks with higher scores are displayed more prominently, _e.g.,_ by being shown higher up on the list when sorted by descending order of priority, a higher priority score is more likely to be on the user's mind. Consequently, the user is more likely to begin processing it mentally, whether intentionally or otherwise, and might even make connections on it to the task currently at hand; the latter is especially useful when the current task is a dependency of a high-priority blocked task, which should be reflected by the priority as well.


# Plugin system

## Standalone binaries

Plugins, such as file format parsers, can take the form of standalone binaries, communicating to `moirai` via `stdin`/`stdout`, or even IPC sockets (in the case of a daemon mode).

`stdin`-style plugins are not merely theoretical: see [kakoune-lsp/kakoune-lsp](https://github.com/kakoune-lsp/kakoune-lsp) .

### Advantages

- A plugin is also a tool; its usefulness extends beyond `moirai`.

### Disadvantages

The main disadvantage of this is the communication protocol. Concretely, this takes the form of two issues:

1. How does `moirai` know that the external tool speaks the same format it does?
  - This is partly solvable by implementing an adapter. It does not matter that `moirai` and the plugin does not speak the same language, as long as there is a translation layer that facilitates such communication. If the communication medium is plaintext (_e.g._ via `stdin`), it can be as trivial as a shell script leveraging tools such as `jq`. The issue is that this adds an extra component to maintain, on top of the added overhead of a translation layer, worsening the bottleneck.
2. `stdin` is inherently untyped; how does `moirai` know that the external tool will give it an integer and not a string? This is somewhat mitigated by good parsing hygiene, but it still injects a point of fallibility, which is less than ideal.


### Bottlenecks

This approach is bottlenecked by the IPC throughput. Since it's a native binary, plugins are able to achieve bare-metal performance. However, having speedy computation is meaningless if it cannot relay the results to `moirai` at a comparable rate.

Plaintext is arguably the slowest; it is not even comparable to approaches such as [Protobuf](https://capnproto.org/). However, what it lacks in terms of speed, it makes up in terms of portability. With formats such as JSON, it becomes usable in conjunction with other tools and scripts, not just with `moirai`. We can even leverage [simdjson](https://docs.rs/simd-json/latest/simd_json/) in order to speed up deserialisation.

## WASM plugins

This is similar to the plugin system used by [Zellij](https://zellij.dev/documentation/plugins.html) or [Zed](https://zed.dev/docs/extensions/developing-extensions).

This approach is effectively a mirror opposite to the standalone binary approach in terms of tradeoff. It is bottlenecked more by computation speed rather than communication (as WebAssembly imposes some performance penalty in exchange for portability), and it provides strong guarantee in terms of communication protocol (in the form of [WASI](https://github.com/WebAssembly/WASI)), in return for plugins not being usable without `moirai`.

A good library to implement this is [Extism](https://extism.org/).

# Client-server architecture

This project is intended to be a server, in two capacities:

- an LSP server, providing integration with arbitrary editors with LSP support, and
- as a backend; a web frontend is planned in the future, providing an Obsidian-esque user interface.

# Todos

- Look into using ropes to provide faster editing: <https://www.cs.tufts.edu/comp/150FP/archive/hans-boehm/ropes.pdf>.
- Interesting notes on CRDTs: <https://github.com/xi-editor/xi-editor/issues/1187#issuecomment-491473599>.
- Look into [MessagePack](https://msgpack.org/index.html) as a communication protocol for standalone plugins.
