import vizhu
import time

# The function user_search take two Arguments:
#   (string, _) should be the username you would like to search
#   (_, boolean) tells whether or not to write the output to a file 
vizhu.user_search("drkrssll", False)

# SnusProps is a Class that takes 3 string arguments
props = vizhu.SnusProps("your_api_key", "email", "example@gmail.com")

# After passing through the arguments, call the snusbase() Method
props.snusbase(False)

# When using in a loop, consider waiting between each request
# This can help prevent rate limiting
time.sleep(2)

count = 0

while count > 3:
    loop_props = vizhu.SnusProps("your_api_key", "user", "drkrssll")

    loop_props.snusbase(False)
    time.sleep(3)

    count += 1
