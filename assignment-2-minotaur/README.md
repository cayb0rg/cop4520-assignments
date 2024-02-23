# Programming Assignment 2

## Problem 1: Minotaur’s Birthday Party

The Minotaur invited N guests to his birthday party. When the guests arrived, he made
the following announcement:

> "The guests may enter his labyrinth, one at a time and only when he invites them to do so. At the end of the labyrinth, the Minotaur placed a birthday cupcake on a plate. When a guest finds a way out of the labyrinth, he or she may decide to eat the birthday cupcake or leave it. If the cupcake is eaten by the previous guest, the next guest will find the cupcake plate empty and may request another cupcake by asking the Minotaur’s servants. When the servants bring a new cupcake the guest may decide to eat it or leave it on the plate."

The Minotaur’s only request for each guest is to not talk to the other guests about her or his visit to the labyrinth after the game has started. The guests are allowed to come up with a strategy prior to the beginning of the game. There are many birthday cupcakes, so the Minotaur may pick the same guests multiple times and ask them to enter the labyrinth. Before the party is over, the Minotaur wants to know if all of his guests have had the chance to enter his labyrinth. To do so, the guests must announce that they have all visited the labyrinth at least once.

Now the guests must come up with a strategy to let the Minotaur know that every guest entered the Minotaur’s labyrinth. It is known that there is already a birthday cupcake left at the labyrinth’s exit at the start of the game. How would the guests do this and not disappoint his generous and a bit temperamental host?

Create a program to simulate the winning strategy (protocol) where each guest is represented by one running thread. In your program you can choose a concrete number for N or ask the user to specify N at the start.

## Problem 2: Minotaur’s Crystal Vase

The Minotaur decided to show his favorite crystal vase to his guests in a dedicated showroom with a single door. He did not want many guests to gather around the vase and accidentally break it. For this reason, he would allow only one guest at a time into the showroom. He asked his guests to choose from one of three possible strategies for viewing the Minotaur’s favorite crystal vase:

1) Any guest could stop by and check whether the showroom’s door is open at any time and try to enter the room. While this would allow the guests to roam around the castle and enjoy the party, this strategy may also cause large crowds of eager guests to gather around the door. A particular guest wanting to see the vase would also have no guarantee that she or he will be able to do so and when.

2) The Minotaur’s second strategy allowed the guests to place a sign on the door indicating when the showroom is available. The sign would read “AVAILABLE” or “BUSY.” Every guest is responsible to set the sign to “BUSY” when entering the showroom and back to “AVAILABLE” upon exit. That way guests would not bother trying to go to the showroom if it is not available.

3) The third strategy would allow the quests to line in a queue. Every guest exiting the room was responsible to notify the guest standing in front of the queue that the showroom is available. Guests were allowed to queue multiple times.

Which of these three strategies should the guests choose? Please discuss the advantages and disadvantages.

Implement the strategy/protocol of your choice where each guest is represented by 1 running thread. You can choose a concrete number for the number of guests or ask the user to specify it at the start.

## Instructions

Make sure you have Rust installed.

Run the following:

```[language=bash]
git clone git@github.com:cayb0rg/cop4520-assignments.git
&& cd assignment-2-minotaur
```

The program takes in two arguments: `cargo run [problem_number] [number_of_threads]`

### Problem 1

```[language=Rust]
cargo run 1 8
```

The program will output the current state of its guests, and will exit once all guests have eaten a cupcake.

### Problem 2

```[language=Rust]
cargo run 2 8
```

The program will output the current state of its guests, and will not exit until you press `Ctrl+C` because the original problem did not state the exit case.

## Problem 1 Strategy

These are the rules the guests decide on BEFORE entering the labyrinth:

1. If it's their first time and there is a cupcake, eat it
2. If it's their first time and there is no cupcake, do nothing
3. If they've been in before and there's a cupcake, do nothing
4. If they've been in before and there's no cupcake, do nothing
5. One person is the counter. If the counter enters the labyrinth and there is no cupcake, they replace it and increase the count by one. Once the counter reaches a count of all the number of guests (minus themselves), the counter tells the minotaur that all guests have visited the labyrinth.
6. If the counter enters the labyrinth and there is a cupcake, they do nothing.

## Problem 2 Answers

### Strategy 1

The first strategy doesn't implement mutual exclusion since any guest can try to enter the room, meaning multiple guests could be in the room at the same time. This method also does not have freedom from starvation. A guest could be waiting forever and never get in, while other guests enter the room several times.

### Strategy 2

The second strategy prevents guests from waiting outside the room to get in (they don't try to call lock() if the door is `BUSY`). This maintains mutual exclusion by using a lock (represented by the sign). However, if a door reads `AVAILABLE`, but two guests try to enter at the same time, then they might be waiting forever for each other (deadlock). Guests might also waste time checking the sign to see if it is `AVAILABLE` many times.

### Strategy 3

The third strategy has mutual exclusion, freedom from deadlock, and freedom from starvation. Every guest will eventually make it into the room, and it's not possible for two or more guests to get into a deadlock since it is queue-based (FIFO). However, because guests can queue multiple times, some guests may spend more time in the room than others. Also, the queue requires additional data structures that consumes computational resources.
