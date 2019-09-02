''' CSV implicit header: domain, salt, encoding, length, optional username '''
import os
import sys
from getpass import getpass
from base64 import *
from hashlib import scrypt

CONFIG = f'{os.environ["HOME"]}/.config/spm/'


def get_records(domain):
    result = []
    with open(f'{CONFIG}state', 'r') as file:
        file = file.read().split('\n')
        for row in file:
            if row.startswith(domain + ','):
                result.append(row)
            elif result:
                break
    return result


def store_record(domain, salt, base, length, username=''):
    salt = b85encode(salt).decode('utf-8')
    record = f'{domain},{salt},{base[-2:]},{length},{username}'
    with open(f'{CONFIG}state', 'r') as file:
        file = file.read().split('\n')
        for i in range(0, len(file)):
            if record < row:
                file.insert(i, record)
                break
        else:
            file.append(record)
    with open(f'{CONFIG}state', 'w') as dest:
        dest.write('\n'.join(file))


def calculate_password(salt, base, length):
    with open(f'{CONFIG}seed', 'r') as f:
        seed = bytes(f.read(), 'utf-8')
    pin = bytes(getpass("Input your PIN: "), 'utf-8')
    key = scrypt(seed + pin, salt=salt, n=16384, r=8, p=1)
    assert base[-2:] in ['16', '32', '64', '85']
    return eval(f'b{base[-2:]}encode({key})').decode('utf-8')[: int(length)]


def generate_password(domain, length, base, username):
    salt = os.urandom(16)
    store_record(domain, salt, base, length, username)
    calculate_password(salt, base, length)


def print_records(records):
    for rec in records:
        rec = rec.split(',')
        rec[1] = b85decode(rec[1])
        if rec[-1]:
            print(f'Username: {username}, ', end='')
        print(f'Password: {calculate_password(rec[1], rec[2], rec[3])}')


def main():
    if sys.argv[1] == 'generate':
        user = sys.argv[5] if len(sys.argv) > 5 else ''
        generate_password(sys.argv[2], sys.argv[3], sys.argv[4], user)
    elif sys.argv[1] == 'get':
        records = get_records(sys.argv[2])
        if not records:
            print('Error: no records found for that domain')
            return
        print_records(records)


if __name__ == '__main__':
    main()
