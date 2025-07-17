MAIN:
    lui  s1, 0xFFFFF
    ori  s2, zero, 1
    ori  s3, zero, 2
    ori  s4, zero, 3
	ori  t0, zero, 1
	slli t0, t0, 25
	sw   t0, 0x24(s1)           # Set frequency of the Timer

L1:
	lw   t0, 0x20(s1)           # Read value of the Timer
	sw   t0, 0x00(s1)           # Write 7-seg LEDs
    
    lw   t1, 0x70(s1)           # Read switches
    andi t1, t1, 3              # Extract the last 2 bits of switches
    beq  t1, s2, L2             # Check if SW[1:0] == 01
    jal  L1
    
L2:
	lw   s5, 0x20(s1)           # Read value of the Timer
	sw   s5, 0x00(s1)           # Write 7-seg LEDs
	
L3:
	lw   t1, 0x70(s1)           # Read switches 
	andi t1, t1, 3              # Extract the last 2 bits of switches
	beq  t1, s3, L4             # Check if SW[1:0] == 10 
	jal  L3
	
L4:
	srli t1, s5, 31             # Extract bit 31 of seed
	
	slli t2, s5, 10             # Extract bit 21 of seed
	srli t2, t2, 31
	
	slli t3, s5, 30             # Extract bit 1 of seed
	srli t3, t3, 31
	
	andi t4, s5, 1              # Extract bit 0 of seed
	
	xor  t1, t1, t2             # xor the four bits
	xor  t1, t1, t3
	xor  t1, t1, t4
	
	slli s5, s5, 1              # generate the new seed
	or   s5, s5, t1
	
	sw   s5, 0x00(s1)           # write the new seed to 7-seg LEDs
	
	lw   t1, 0x70(s1)           # Read switches 
	andi t1, t1, 3              # Extract the last 2 bits of switches
	beq  t1, s4, L5             # Check if SW[1:0] == 11 
	jal  L4
	
L5:
	ori  s2, zero, 7            # set i = 7
	jal  L7

L6:
	add  s3, zero, zero         # set j = 0
L9:
	addi s4, s3, 1              # set k = j + 1
	
	slli s6, s3, 2              # calculate j * 4
	slli s7, s4, 2              # calculate k * 4
	
	srl  t1, s5, s6             # Extract A[j]
	andi t1, t1, 0xf
	srl  t2, s5, s7             # Extract A[k]
	andi t2, t2, 0xf
	
	blt  t2, t1, SWAP           # Compare A[j] and A[k]
	
L8:
	srl  t5, s5, s6             # construct the new number
	srli t5, t5, 8
	slli t5, t5, 4
	or   t5, t5, t2
	slli t5, t5, 4
	or   t5, t5, t1
	sll  t5, t5, s6
	
	addi t3, zero, 1
	sll t3, t3, s6
	addi t3, t3, -1
	and  t3, t3, s5
	
	or   t5, t5, t3
	add  s5, zero, t5
	
	addi s3, s3, 1              # set j = j + 1
	blt  s3, s2, L9             # if j < i
	addi s2, s2, -1             # set i = i - 1
L7:
	blt  zero, s2, L6           # if i > 0
	
	addi t1, zero, 1            # turn on LED 0
	sw   t1, 0x60(s1)
	
L11:
	lw   t1, 0x70(s1)           # Read switches 
	andi t1, t1, 3              # Extract the last 2 bits of switches
	beq  t1, zero, L10          # Check if SW[1:0] == 00
	jal L11                     
	
L10:
	sw   s5, 0x00(s1)           # Wirte the sorted numbers to 7-seg LEDs
	jal EXIT
	
SWAP:
	xor t1, t1, t2
	xor t2, t1, t2
	xor t1, t1, t2
	jal L8
	
EXIT:
	jal EXIT
