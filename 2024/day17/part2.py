from z3 import *


a = BitVec('a0',128)
b = BitVec('b0',128)
c = BitVec('c0',128)


target =  "2,4,1,1,7,5,1,5,0,3,4,3,5,5,3,0".split(",")
target = [int(x) for x in target]
print(target)

s = Optimize()

ac = a
bc = b
cc = c

s.add(b ==0)
s.add(c ==0)

for i in range(len(target)):

    s.add(ac != 0)
    bc = ac % 8
    bc = bc ^ BitVecVal(1,128)
    cc = ac >> bc
    bc = bc ^ BitVecVal(5,128)
    ac = ac >> 3
    bc = bc^cc
    s.add(bc % 8 == target[i])

s.add(ac == 0)
s.minimize(a)
if s.check() == sat:
    print(s.model())
print(s.check())


