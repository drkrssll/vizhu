import vizhu

# The function user_search take two Arguments:
#   (string, _) should be the username you would like to search
#   (_, boolean) tells whether or not to write the output to a file 
vizhu.user_search("drkrssll", False)

# SnusProps is a Class that takes 3 string arguments
props = vizhu.SnusProps(api_key, "email", "example@gmail.com")

# After passing through the arguments, call the snusbase() Method
props.snusbase()
