# Vizhu
Vizhu is a Python library designed for Open Source Intelligence (OSINT) investigations. It allows you to search for usernames across multiple social media platforms effortlessly.

![Example](vizhu.png)

## Features
- Username Search: Check the availability or existence of a username across various social media platforms.
- Flexible Output: Choose to print results to the console or write them to a log file.
- Fast and Efficient: Leveraging Rust's performance for quick searches.

## Installation
Ensure you have Python 3.7 or higher and Rust installed on your system.

Clone the Repository
```sh
git clone https://github.com/yourusername/vizhu.git
cd vizhu
```

Set Up a Virtual Environment (Optional but Recommended)
```sh
python -m venv env
source env/bin/activate
```

Install Maturin
```sh
pip install maturin
```

Build and Install Vizhu
```sh
maturin develop
```

## Usage
To use Vizhu in your Python projects:
```py
import vizhu

# Replace 'some_username' with the username you want to search for
vizhu.user_search('some_username', False)

# Or write the results to a log file
vizhu.user_search('some_username', True)
```

### Function Parameters
- username (str): The username you want to search for.
- write_to_file (bool):
    > - False: Prints the search results to the console.
    > -  True: Writes the search results to a timestamped log file in the logs directory.

### Console Output:
```
[200]: https://instagram.com/johndoe
[404]: https://github.com/johndoe
[200]: https://x.com/johndoe
...
```

#### Searches for accounts on:
> - Instagram
> - GitHub
> - Twitter (X)
> - Reddit
> - TikTok
> - Imgur
> - Facebook
> - Pinterest
> - Telegram
> - Tumblr

## Contributing
Contributions are welcome! Please open an issue or submit a pull request on the GitHub repository.

