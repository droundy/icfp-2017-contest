# icfp-2017-contest
Entry for the 2017 ICFP programming contest

## Plan

1. Write a python program that executes an offline program and
   connects it with a server.  This way we will not need to write
   networking into our punter.

2. A python punter, which will be quick to write and allow us to
   experiment.  Also it'll give us something to test our punter
   against, and a place to try new algorithms..

3. A rust punter.  This looks very speed-intensive, so this will
   probably be our final entry.

4. An offline-mode game runner.  This might not be worth the time, but
   the process of writing it would involve much of the same logic that
   a good punter would require.

5. A game visualizer.  Yes, they offer theirs, but it might be more
   fun to also write our own!

1. A Battle Royale program that plays players against one another and
   gives a composite score. Replica tes as closely as possible the
   organizers scoring.

1. NOTE: we should handle swapped source and target numbers.

## Plan for punter code:

6. Spawn optimizers in try threads, which store anything they find in
   a Mutex. At time limit sends the best move.

1. Add ability to store in state the future sequence of moves along
   with their values. Spend the entire time planning ahead in every
   case.

1. Need a metric of goodness so each optimizer can quantify how good
   their answer is.  This could be a tuple of depth searched with
   expected score. Maybe maximum score also? Possibly also with some
   information indicating how reliable the expected scores is.  In
   practice, we should probably write multiple metrics, since this is
   a challenging heuristic.

1. A command line interface that allows to pick the set of optimizers
   and valuers to run, to be used by the Battle Royale program.

### Ideas for optimizers

1. One optimizer just picks first legal move.

1. One optimizer that is eager.

1. One optimizer is exhaustive, but gives up on big maps, ie ones with
   many rivers unclaimed.

1. One optimizer uses a simple heuristic picking the option that will
   score best without interference.

1. One optimizer only works for two player game but values
   interference equally with scoring.

1. A Monte Carlo optimizer? Works at full depth, but tweaks moves.

1. An optimizer that picks a random river.

1. An optimizer that creates a Future and focuses specifically on building a
   path to that future.

1. An optimizer that attempts to build a long path between many mines.

1. An optimizer that identifies and prioritizes "bottlenecks".

### Ideas for valuers

1. Possibly a heuristic board valuing function? Several heuristics?
   This would take a given board and guess at the anticipated score
   somehow.

1. A Monte Carlo valuer? Could pick random moves or something to
   determine an estimated score.

1. A valuer that uses a simple board heuristic, and alternates moves
   that give the best board heuristic.

1. Code that tests different heuristics by pitting simple algorithms
   based on each heuristic against each other.  i.e. the same
   algorithm with different heuristics can be competed against one
   another.



## Testing the rust code

You can run the setup stage with

    cargo run < examples/setup.sample

You can run a gameplay step with

    cargo run < examples/gameplay

Neither does anything special, but both should parse the json and
output valid json.

## Team members

David Roundy <roundyd@physics.oregonstate.edu>

Garrett Jepson <jepsong@oregonstate.edu>
