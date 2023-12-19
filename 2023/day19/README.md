# Day 19

## Task 1
Pretty straightforward. Parse each Workspace into a Vector of Rules, where each rule either marks a range (in 1-4000) as valid. Or is the default rule.
Then save Workspaces into a HashMap : &str -> \[Rules\].
Parse the parts as Hashmap : Char -> i64 and iterate over all of them, starting in "in", until they are either accepted or rejected.

## Task 2
Parse Workspaces as before. 
Parts now store a Hashmap : Char -> Range
Then FIFO Queue starting with a Tuple ("in", P) where P is a Part with range 1-4000 for each of the 4 ratings.
While Elements in Queue. Pop first element.
Apply the first rule that matches and split the part into parts, depending on the destination and range of the rule.
If Part is accepted, or rejected, add it to the corresponding vector. If it needs to be further processed, add it to the back of the queue.

After the queue is empty, multiply the length of ranges for each accepted part and add them together for the Result.
