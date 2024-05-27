import sys

def setup():
    print("global setup...");

def teardown():
    print("global teardown...");

if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "setup":
            setup()
        elif sys.argv[1] == "teardown":
            teardown()
