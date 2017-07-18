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

### Bit vector

The members are stored as bits set in a bit vector. The vector has a storage bit for each allowed member.

[Bit array](https://en.wikipedia.org/wiki/Bit_array)

## Preliminary Results

All the members and tested valus are generated randomly.

-------------
### Set size: 200000, Tested values: 100
#### Members: 100000

* Linear search  took: 4672210 ns
* Binary search took: 24788 ns
* Cuckoo search took: 13219 ns
* Hash search took: 3580 ns
* Bit search took: 230 ns

#### Members: 10000

* Linear search  took: 671335 ns
* Binary search took: 8669 ns
* Cuckoo search took: 7239 ns
* Hash search took: 2440 ns
* Bit search took: 220 ns

#### Members: 1000

* Linear search  took: 68754 ns
* Binary search took: 5800 ns
* Cuckoo search took: 5619 ns
* Hash search took: 2150 ns
* Bit search took: 180 ns

#### Members: 100

* Linear search  took: 7169 ns
* Binary search took: 3770 ns
* Cuckoo search took: 5049 ns
* Hash search took: 2210 ns
* Bit search took: 190 ns

#### Members: 10

* Linear search  took: 1270 ns
* Binary search took: 2250 ns
* Cuckoo search took: 7469 ns
* Hash search took: 2340 ns
* Bit search took: 260 ns

--------------

## TODO
- Proper visualizations with different set sizes
- Include enough repetitions
- Document space requirements for different approaches
- Bloom filter
- Static hash set implemented with a single array to allow easy serialization.

