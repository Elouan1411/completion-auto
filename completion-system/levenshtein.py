# def optimized_levenshtein(a, b):
# n, m = len(a), len(b)
#     	prev = list(range(m + 1))
#     	curr = [0] * (m + 1)

#     	for i in range(1, n + 1):
#         		curr[0] = i
#        		 for j in range(1, m + 1):
#            			 insert = curr[j - 1] + 1
#            			 delete = prev[j] + 1
#             			substitute = prev[j - 1] + (0 if a[i - 1] == b[j - 1] else 1)
#             			curr[j] = min(insert, delete, substitute)
#         		prev, curr = curr, prev

#    	 return prev[m]


def levenshtein_distance(a, b):
    n, m = len(a), len(b)
    dp = [[0] * (m + 1) for _ in range(n + 1)]

    for i in range(n + 1):
            for j in range(m + 1):
                    if i == 0:
                        dp[i][j] = j
                    elif j == 0:
                        dp[i][j] = i
                    elif a[i - 1] == b[j - 1]:
                        dp[i][j] = dp[i - 1][j - 1]
                    else:
                        dp[i][j] = 1 + min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1])

    return dp[n][m]


print(levenshtein_distance("bonjour", "bon"))  # 3