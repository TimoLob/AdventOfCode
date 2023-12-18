# Day 17
This was hard. It's just a modified pathfinding algorithm, but I couldn't wrap my head around the graph structure.
Insead of just using (x,y) as Nodes, I use (x,y, Direction we entered the node from, steps in the same direction since last turn) as graph nodes.
It took a while to figure out this graph structure and every movement possibility, but I got there.