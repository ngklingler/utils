import base64
import hashlib
import json
import os
import sys

CONFIG = f'{os.environ["HOME"]}/.config/spm/'


def get_record(domain):
    with open(f'{CONFIG}state', 'r') as f:
        state = json.load(f)
        record = state[domain]
        record['salt'] = base64.b85decode(record['salt'])
    return record


def store_record(domain, salt, encoding, length):
    with open(f'{CONFIG}state', 'r') as f:
        state = json.load(f)
    state[domain] = {
        'salt': base64.b85encode(salt).decode('utf-8'),
        'encoding': encoding,
        'length': length,
    }
    with open(f'{CONFIG}state', 'w') as f:
        json.dump(state, f)


def calculate_password(salt, encoding, length):
    if type(length) == str:
        length = int(length)
    with open(f'{CONFIG}seed', 'r') as f:
        seed = bytes(f.read(), 'utf-8')
    # TODO use getpass
    pin = bytes(input("Input your PIN: "), 'utf-8')
    key = hashlib.scrypt(seed + pin, salt=salt, n=16384, r=8, p=1)
    assert encoding[4:] in ['16', '32', '64', '85']
    print(
        eval(f'base64.b{encoding[4:]}encode({key})').decode('utf-8')[:length]
    )


def generate_password(domain, length, encoding):
    salt = os.urandom(16)
    store_record(domain, salt, encoding, length)
    calculate_password(salt, encoding, length)


def init(how):
    os.system(f'[ -f {CONFIG}state ] || echo {{}} > {CONFIG}state')
    if how == 'new':
        os.system(f'mkdir -p {CONFIG}')
        os.system(f'pgen > {CONFIG}seed')
        print("Here is your 12 word seed. Please record it.")
        os.system(f'cat {CONFIG}seed')
    elif how == 'restore':
        seed = input('Enter your seed: ')
        with open(f'{CONFIG}seed', 'w') as f:
            f.write(seed)
    else:
        print("Error, unrecognized parameter passed to init")


def main():
    if sys.argv[1] == 'init':
        init(sys.argv[2])
    elif sys.argv[1] == 'gen':
        generate_password(sys.argv[2], sys.argv[3], sys.argv[4])
    elif sys.argv[1] == 'get':
        record = get_record(sys.argv[2])
        calculate_password(
            record['salt'], record['encoding'], record['length']
        )


if __name__ == '__main__':
    main()
