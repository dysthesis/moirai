# Description

A description is a string, which may optionally reference tasks, projects, or items (by identifier).

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
