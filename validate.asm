call point:
0x154b: 0x0008 (JF)   r7      0x15e5  
// snip - just some code outputting text
0x156b: 0x0001 (SET)  r0      0x0004  
0x156e: 0x0001 (SET)  r1      0x0001  
0x1571: 0x0011 (CALL) 0x178b  
0x1573: 0x0004 (EQ)   r1      r0      0x0006  
0x1577: 0x0008 (JF)   r1      0x15cb  

in()
0x178b: 0x0007 (JT)   r0      0x1793  
0x178e: 0x0009 (ADD)  r0      r1      0x0001  
0x1792: 0x0012 (RET)  

p1()
0x1793: 0x0007 (JT)   r1      0x17a0  
0x1796: 0x0009 (ADD)  r0      r0      0x7fff  
0x179a: 0x0001 (SET)  r1      r7      
0x179d: 0x0011 (CALL) 0x178b  
0x179f: 0x0012 (RET)  

p2()
0x17a0: 0x0002 (PUSH) r0      
0x17a2: 0x0009 (ADD)  r1      r1      0x7fff  
0x17a6: 0x0011 (CALL) 0x178b  
0x17a8: 0x0001 (SET)  r1      r0      
0x17ab: 0x0003 (POP)  r0      
0x17ad: 0x0009 (ADD)  r0      r0      0x7fff  
0x17b1: 0x0011 (CALL) 0x178b  
0x17b3: 0x0012 (RET)  

On entry
r0: 4
r1: 1
r7: whatever we set it to

Goal r0 = 6 on final return