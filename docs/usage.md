# Usage

## **Key Generation**

Keys can be generated generated using the `-k` flag. A size and a round count should be provided. A 15kb key can be used on any file less than 15kb in size. Extra rounds will add security but may take longer to compute.

**Example**:  
`> slick -k 15000 16`

**Output**:
```
15000-byte key:

ABAAAAAAAAA6mOkqfA+Es1L8mVfVNyKYsFE=
```


---

## **Encryption**

Encryption is done using the `-e` flag. A key and file location is needed.

**Example**:  

`> slick -e ABAAAAAAAAA6mOkqfA+Es1L8mVfVNyKYsFE= file.txt`


---

## **Decryption**

Decryption can be accomplished in the same way as encryption using the `-d` flag.

**Example**:  
`> slick -d ABAAAAAAAAA6mOkqfA+Es1L8mVfVNyKYsFE= file.txt`


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

Times for generating encrypted vector of various length can be observed using the `-t` flag and a start and end length. This can also be plotted using Gnuplot as the output is in csv format. A script also exists to automatically test and fit time data.

**Example**:  

`> slick -t 10000 25000`  
`> slick -t 10000 25000 | gnuplot -p -e 'plot "/dev/stdin"'`   
`> sh time.sh 10000 25000`