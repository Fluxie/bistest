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
The set members of the set are stored in an array in arbitrary order. The membership is tested by scanning the whole memberhip array.

[Linear search](https://en.wikipedia.org/wiki/Linear_search)

### Binary search
The set members of the set are stored in a sorted array. The memberhip is tested with a binary search from the array.

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

### Array based hash set

The set members and bookkeeping information of the hash set are stored in a single vector. The current implementation uses a simple modulo operator as the hashing function.

Vector's contents:

| # buckets | index to the first value in the 1st bucket | index to the first value in the 2nd bucket | ... | index to the first value of the last bucket | size of the whole vector | 1st bucket | 2nd bucket | ... | last bucket |

The size of the whole vector is included to simplify the implementation. The values of each bucket are sorted to enable binary search.

When testing if a given value is a member of the set, the bucket is determined with the modulo operation. After finding the correct bucket, a binary search is applied to the bucket to determine if the tested value is actually in the bucket.

The current implementation splits the members into buckets so that each bucket holds on average 5% of the values. The space overhead is then also 5%. The lookup speed and space overhead can be adjusted by adjusting the number of buckets relative to the number of set members and the upper bound of the possible values in the set.


### Hash set/table lookup
The set members are stored in a hash set / table provided by the Rust standard libray and the membership is tested with a simple lookup to the hash set.

[Hash table](https://en.wikipedia.org/wiki/Hash_table)

### Bit vector

The members are stored as bits set in a bit vector. The vector has a storage bit for each allowed set member.

[Bit array](https://en.wikipedia.org/wiki/Bit_array)

## Preliminary Results

All the set members and the tested valus are generated randomly.

-------------
### Set size: 200000, Tested values: 100
#### Members: 100000

* Linear search  took: 3784944 ns
* Binary search took: 29267 ns
* Cuckoo search took: 15629 ns
* Array hash search took: 4430 ns
* Hash search took: 3520 ns
* Bit search took: 229 ns

#### Members: 10000

* Linear search  took: 547165 ns
* Binary search took: 10619 ns
* Cuckoo search took: 7340 ns
* Array hash search took: 3960 ns
* Hash search took: 2680 ns
* Bit search took: 190 ns

#### Members: 1000

* Linear search  took: 58716 ns
* Binary search took: 6839 ns
* Cuckoo search took: 5329 ns
* Array hash search took: 3799 ns
* Hash search took: 2030 ns
* Bit search took: 170 ns

#### Members: 100

* Linear search  took: 6679 ns
* Binary search took: 4720 ns
* Cuckoo search took: 5199 ns
* Array hash search took: 3500 ns
* Hash search took: 2350 ns
* Bit search took: 180 ns

#### Members: 10

* Linear search  took: 920 ns
* Binary search took: 2460 ns
* Cuckoo search took: 5070 ns
* Array hash search took: 720 ns
* Hash search took: 1419 ns
* Bit search took: 170 ns


--------------

## TODO
- Proper visualizations with different set sizes
- Include enough repetitions
- Document space requirements for different approaches
- Bloom filter
- Static hash set implemented with a single array to allow easy serialization.

