# Installation

## **Install**

First, clone the repo:  
`> git clone git@github.com:mbednarek360/SLICK.git`

Then compile using cargo:  
`> cd SLICK`  
`> cargo build --release`

Finally, move the compiled binary:  
`> sudo mv target/release/SLICK /usr/bin/slick`

You may then remove the cloned repo:  
`> cd ..`
`rm -rf SLICK`

---

## **Uninstall**

To uninstall, simply remove the binary:  
`> sudo rm /usr/bin/slick`