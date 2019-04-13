# Usage

## **Key Generation**

Keys can be generated generated using the `-k` flag. The only argument needed is the max file size to be used. A 50 byte key can be used on any file less than 50 bytes in size.

**Example**:  
`> slick -k 50`

**Output**:
```
50-byte key:

32xA5D667F9C91B8AD3DC3DA3F9E8A7207E
```


---

## **Encryption**

Encryption is done using the `-e` flag. A key and file location is needed.

**Example**:  

`> slick -e 32xA5D667F9C91B8AD3DC3DA3F9E8A7207E file.txt`


---

## **Decryption**

Decryption can be accomplished in the same way as encryption using the `-d` flag.

**Example**:  
`> slick -d 32xA5D667F9C91B8AD3DC3DA3F9E8A7207E file.txt`


---

## **Index Vector Generation**

Calculating all shuffled index vectors for a given length can be done with the `-p` flag.

**Example**:  
`> slick -p 3`

**Output**:
```
[1, 2, 0]
[2, 0, 1]
[1, 0, 2]
[2, 1, 0]
[0, 2, 1]
```

---

## **Speed Test**

Times for generating shuffled index vectors of various length can be observed using the `-t` flag and a length to go up to. This can also be plotted using gnuplot.

**Example**:  

`> slick -t 25000`  
`> slick -t 25000 | gnuplot -p -e 'plot "/dev/stdin"'`