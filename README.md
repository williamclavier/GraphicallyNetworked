# Graphically Networked
This is a project to analyze the network connectivity of users on Twitter from an anonymized data set of users. 
[Full Report](https://github.com/williamclavier/GraphicallyNetworked/blob/main/Final_Report.pdf)
## Installation
```bash
git clone https://github.com/williamclavier/GraphicallyNetworked.git
cd GraphicallyNetworked
```
## Execution
```bash
cargo run --release
```
## Expected Output:
```
For samples of size: 10000
i=0     Average node distance: 5.547644334912133
i=1     Average node distance: 5.567798345126696
i=2     Average node distance: 5.560872420680197
i=3     Average node distance: 5.565487214677027
i=4     Average node distance: 5.564177950239673
```
## Data Source
J. McAuley and J. Leskovec. [Learning to Discover Social Circles in Ego Networks](http://i.stanford.edu/~julian/pdfs/nips2012.pdf). NIPS, 2012.
Accessed through: https://snap.stanford.edu/data/ego-Twitter.html