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

# Conclusions

## Near bit 14

* Either bit 14, or two bits 13, causes bit 15 to be set
* This indicates that z14 is swapped with something related to bit 15
* Currently, z14 is the output of an OR! So it must be swapped with c14 = vss

## Near bit 22

* Either bit 22 results in 23 being set
* Both bits 22 result in bit 22 being set. So likely one of the carry wires is swapped.
* Some wires in this area:
  * y22 XOR x22 -> hjf
  * y22 AND x22 -> kdh
  * bjb OR hjf -> sbg
  * kdh XOR gjn -> z22
* kdf and hjf appear swapped

## Near bit 30

* Either bit 31, or both bits 30, sets bit 32
* Some wires in this area:
  * x31 XOR y31 -> nrr
  * nrr XOR sms -> kpp
  * nrr AND sms -> z31
* kpp and z31 are swapped

## Near bit 35

* Either bit 35 (or both 34) result in setting bit 36
* Wires in the area:
  * y35 AND x35 -> z35
  * y35 XOR x35 -> bbc
  * bbc XOR jkb -> sgj
