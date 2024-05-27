# tests/local/module_test.py
import sys

def setup():
    print("local testsuite setup...");

def teardown():
    print("local testsuite teardown...");

if __name__ == "__main__":
    if len(sys.argv) > 1:
        if sys.argv[1] == "setup":
            setup()
        elif sys.argv[1] == "teardown":
            teardown()
