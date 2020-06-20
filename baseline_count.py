import sys

if __name__ == '__main__':
    with open(sys.argv[1], 'r') as ifile:
        count = 0 
        for _ in ifile:
            count += 1
        print(count)
