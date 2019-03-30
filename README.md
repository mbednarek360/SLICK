# SLICK

**S**huffled
**L**ine
**I**nteger
**C**iphering
**K**ey

---
### **Key Generation:** 
Random number in the range 1 to l! - 1.  
Can encrypt / decrypt information with a size less than l.  
Length is also encoded into the key.  

Ex 50-byte key: `32xC09CA72CE847FA6A`

---
### **Encryption:**  
Encode data into a binary sequence.  
Apply shift cipher on each byte with location.  
Shuffle encoding by repeatedly swapping adjacent bytes.  