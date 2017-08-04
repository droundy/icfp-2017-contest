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
