## Game Overview
DigiDog is a digital version of the Swiss board game "Dog". It's a team-based game similar to "Eile mit Weile" but with more tactical elements through card play.

## Game Rules

### Basic Rules
- Teams consist of two players sitting opposite each other.
- Each team tries to get all 8 of their pieces from start to finish first.
- Players move pieces clockwise around the board using cards.
- Only teams can win, not individual players.

### Game Flow
1. Each player receives 6 cards at the start.
2. Partners exchange one card face down before each round.
3. Players take turns playing cards and moving pieces accordingly.
4. If a player can't move, they discard all cards and skip the rest of the round.
5. A round ends when all cards are played.
6. In subsequent rounds, card numbers decrease (6, 5, 4, 3, 2) before starting over.

### Special Rules
- Players can only start a piece with an Ace, King, or Joker.
- Players can distribute moves with a Seven. 
- The Seven cards burn all pieces on its trail (moving them back to the start).
- Players can go reverse with a Four.
- When two pieces land on the same field, the second piece sends the first one back to start.
- Players must use every card they play, even if it's disadvantageous.
- To enter the goal area, a piece must have passed its starting position at least twice.

## Setup and Running

Download the jar file

### Starting the Server

```sh
java -jar digidog-0.0.1-ALPHA.jar server <PORT>
```

### Starting the Client
```sh
java -jar digidog-0.0.1-ALPHA.jar client <IP>:<PORT>
```