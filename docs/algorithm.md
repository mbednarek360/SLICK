# How it Works

## **Key Generation**

Each key contains three values, a round count, size, and seed. The seed is used to permute, shift, and substitute a vector. It is generated as a pseudo-random value such that `0 < k < l!` where l is the length of the input, and k is the k. The size value is used to fit the vector to such length, and the round count is used to know how many times to encrypt the vector. The key is encoded first in hex then base64.

**Example**:  

15000-byte key:
```
ABAAAAAAAAA6mOkqfA+Es1L8mVfVNyKYsFE
```
In Hex:
```
0010 0000000000003A98 E92A7C0F84B352FC9957D5372298B051
 ^           ^                        ^
Rounds      Size                     Seed
```

---

## **Index Vector Generation**

A vector of ascending integers spanning the length of the input file is used. Each seed corresponds to an individual permutation of this vector. A single permutation can be calculated in linear time for a given length. This permutation is to be used to determine where each byte will be moved in the encryption / decryption stage.

**Example**:

| Key |    Vector   |
|:---:|-------------|
| 1   | `[1, 2, 0]` |
| 2   | `[2, 0, 1]` |
| 3   | `[1, 0, 2]` |
| 4   | `[2, 1, 0]` |
| 5   | `[0, 2, 1]` |

A single permutation is calculated by looping through the ascending vector. Each iteration a position is calculated using `k % l` where k is the key and l starts as the length of the vector. It then swaps the value at that position with the value at `l - 1`. The final step is to divide the k by l, and then decrement l. This process is repeated while `l > 0`.

--- 

## **Encryption**

Encrypting a file begins with reading it's bytes to a vector. Before permutation, a shuffled byte array of values up to 256 is used to substitute bytes randomly. A index vector is then generated to be used for shuffling those bytes. Iterating through this index vector, the current value is used as an index from which to take a byte from the file vector. Before this byte is to be pushed to the output vector, its value is shifted by `b + i ^ i` were b is the value of the byte and i is the index. This is done so that none of the original bytes are preserved, and the original value for every byte can only be recovered knowing the current index, which is dependent on the key.

---

## **Decryption**

Decryption is done in a similar way to encryption, instead swapping the index and value of the index vector for which to place a given byte. This allows each byte to be returned to its original position, after being shifted back by `b ^ i - i`. Substitution is also reversed by inversly shuffling the byte array.

---

## **Vector Padding**

Before encryption, the vector is padded with 0s so that its original size is not preserved while encrypted. This is also done so that files smaller than the size of the key may be encrypted without generating a new exact key. A byte of value 255 is used to break removal of these 0s.

**Example**:  
```
Size: 5
[104, 101, 108, 108, 111]

Padded: 10 + 1
[104, 101, 108, 108, 111, 255, 0, 0, 0, 0, 0]
```
