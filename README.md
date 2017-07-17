# bistest
Comparison of integer set lookup algorithm speed when the upper bound for the maximum integer value in the set is small.
------
## Assumptions
* Integers are unsigned
* The maximum integer value of the set space is relatively small e.g. < 1 000 000
* Construction of the set happens offline and the results are used multiple times
* The integers that are tested for set membership are in arbitrary order and are used only once.
* The set is immutable once created
------
## Algorithms

### Linear search
The members of the set are stored in an array in arbitrary order. The membership is tested by scanning the whole memberhip array.

[Linear search](https://en.wikipedia.org/wiki/Linear_search)

### Binary search
The members of the set are stored in a sorted array. The memberhip is tested with a binary search from the array.

[Binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm)

### Cuckoo filter
This scheme utilizes two cuckoon filters for fast lookup and a sorted array as a backup. 

Cuckoo filter is a probabilistic data structure. When testing if a given value is member of the set it gives the correct answer
with a certain probability. The test will only return false positives. I.e. if the test outcome is negative then the value
is not part of the set. However, if the the result is positive then the value is in the set with a certain probability.

The two cuckoon filters are constructred from:
- Members of the set.
  - When tested, if this filter returns false we know it is not a member of the set.
- The difference of the whole value space and the set members.
  - When tested, if this filter returns false we know it is a member of the set.
  
If both filters give positive answer for a tested value then the sorted array is used as a backup.

[Cuckoo Filter: Practically Better Than Bloom](https://www.cs.cmu.edu/~dga/papers/cuckoo-conext2014.pdf)

### Hash set/table lookup
The members are stored in a hash set / table and the membership is tested with a search to the hash set.

[Hash table](https://en.wikipedia.org/wiki/Hash_table)

## Preliminary Results

All the members and tested valus are generated randomly.

-------------
### Set size: 200000, Tested values: 100
#### Members: 100000

* Linear search  took: PT0.004733594S
* Binary search took: PT0.000029008S
* Cuckoo search took: PT0.000009849S
* Hash  search took: PT0.000002970S (fastest)

#### Members: 10000

* Linear search  took: PT0.000532958S
* Binary search took: PT0.000009009S
* Cuckoo search took: PT0.000007579S
* Hash  search took: PT0.000004230S (fastest)

#### Members: 1000

* Linear search  took: PT0.000057455S
* Binary search took: PT0.000006740S
* Cuckoo search took: PT0.000006090S
* Hash  search took: PT0.000002040S (fastest)

#### Members: 100

* Linear search  took: PT0.000006620S
* Binary search took: PT0.000004199S
* Cuckoo search took: PT0.000005350S
* Hash  search took: PT0.000002290S (fastest)

--------------

## TODO
- Proper visualizations with different set sizes
- Include enough repetitions
- Document space requirements for different approaches
- Bloom filter
- Static hash set implemented with a single array to allow easy serialization.

