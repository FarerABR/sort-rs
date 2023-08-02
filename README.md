## output of the **`cargo test`**

```
running 10 tests

test sort_test::test_insertion_sort_empty ... ok

test sort_test::test_insertion_sort_one ... ok

test sort_test::test_sort_duplicate_values ... ok

test sort_test::test_sort_large_array ... ok

test sort_test::test_inser ... ok

test sort_test::test_sort_negative_numbers ... ok

test sort_test::test_sort_random ... ok

test sort_test::test_tim_sort_empty ... ok

test sort_test::test_tim_sort_reverse_sorted ... ok

test sort_test::test_tim_sort_sorted ... ok
```

**`test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s`**

---

## well, rust uses 2 sort algorithm:

- Insertion sort
- Timsort

### we all know how insertion sort works and it's only used for slices smaller than 20(lengh), but timsort uses a smart algorithm

### at frist it divides the slice buy an algorithm then it does a mergesort for those slices, this slices then make the final **sorted-slice**
