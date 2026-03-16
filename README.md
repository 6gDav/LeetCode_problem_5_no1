# <a href="https://leetcode.com/problems/longest-palindromic-substring/description/">5. Longest Palindromic Substring</a>

## 📝 Description

Given a string s, return the longest **palindromic substring** in s.

## 🧠 How I solved the problem 

First, I return the original string if it is equal to its reversed version to avoid unnecessary processing.

After this, I implemented a for loop that continuously compares a new string with its own reversed version. The backward logic follows the same principle.

The last one is a bit of a silly attempt, but I'll leave it in.

## ➗ Complexity

* **Time complexity**: *O(n^2)* - Because of the irerations. 
* **Space complexity**: *O(n)* - *dummy_string* and *rev_dummy_string* depends from the lenght of the given texr.

## 📊 Benchmark

I made it in release mode for more accurate results:
```bash
cargo run --release
```

Hardware: *Apple Mac Mini M4*

### 🤏 Small Input Test

* **Execution Time**: *2.167µs*
* **Memory Delta**: *0 bytes*
* **Current Memory**: *1490944 bytes*

### 😖 Stress Test (Large Input)

* **Execution Time**: *2.292µs*
* **Memory Delta**: *0 bytes*
* **Current Memory**: *1523712 bytes*