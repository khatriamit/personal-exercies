## Dynamic programming

    DP is used as an optimization over the plain recursion where ever we see recursive solution we can typically optimize it using the dynamic programming solution

## Longest Common Subsequence

    The Longest Common Subsequence (LCS) is a classic problem in computer science that involves finding the longest subsequence common in two sequences. A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements. For example, given the two sequences "ABCDGH" and "AEDFHR", the LCS is "ADH" with length 3.

    The LCS problem has various applications in computer science, including text comparison, DNA sequence analysis, and version control systems. There are several algorithms to solve the LCS problem, including dynamic programming and recursive approaches.

    The dynamic programming approach involves building a table that stores the lengths of LCS for every prefix of the two sequences. The lengths are calculated based on the previous lengths of shorter prefixes. The final element of the table contains the length of the LCS. The LCS itself can be reconstructed by backtracking through the table.

    The recursive approach involves trying all possible subsequences and comparing them to find the longest common one. This approach has a time complexity of O(2^n), where n is the length of the sequences, and is not practical for long sequences.

    Overall, the LCS problem is an interesting and useful problem in computer science, with various real-world applications.

    Two strings:
        abcd
        defg
            Here our longest common subsequence is 'd' in above two examples

        acio
        boiz
            Here our longest common subsequence is 'io' in above two examples
