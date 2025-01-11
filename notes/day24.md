# What should the circuit look like?

* We have x00 thru x44, and same for y
* We have outputs z00 thru z45, so the carry bit is output
* It looks like our half-adders are XOR based, and we have a ripple-carry adder.
* For each bit BB from 1 thru 45, we'll have wires with logical names:
  * xBB, yBB: Inputs
  * cAA: Carry input from previous adder
  * xBB XOR yBB -> hzBB
  * cAA XOR hzBB -> zBB: Output
  * xBB AND yBB -> caBB
  * hzBB AND cAA -> cbBB
  * caBB OR cbBB -> cBB: Carry output
* Bit zero is special, we have only a half adder
* Bit 45 is special, z45 = c44

# Test inputs

Let's try some test inputs that help us understand what's going on. For each bit i:
* x = just bit i, y = zero
* x = zero, y = just bit i
* x = just bit i, y = just bit i
* x = bits i, i+1; y = bits i; i+1

Conclusions after one round:

