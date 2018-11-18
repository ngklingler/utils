# This is a xonsh script to publish a computer's IP address to a git repository
# It depends on xonsh (visit https://xon.sh), curl, and git

# this are urls that when curled should return the IP address as a plain string
URLS = ['ifconfig.me', 'ipinfo.io/ip', 'ipecho.net/plain', 'icanhazip.com']

# the main function, takes a repository directory (full path),
# a filename to cat the IP address into, parameters to either append the 
# ip address or to truncate, and any notes to be appended after the ip
# address (such as a computer name)
def publish_ip(repo_dir: str,
               filname='ip': str,
               append=True: bool,
               notes='': str) -> bool:
    commit_message = f"publishing IP address{notes}"
    current_ip = ''
    for url in URLS:
        current_ip = $(curl @(url))
        if current_ip:
            break
    if not current_ip:
        return False
    start_dir = $(pwd)[:-1]
    cd @(repo_dir)
    if append:
        echo @(current_ip) >> @(filename)
    else
        echo @(current_ip) > @(filename)
    git add @(filename)
    git commit -m @(commit_message)
    git push
    cd @(start_dir)
    return True    